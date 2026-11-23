use crate::agent::registry::AgentRegistry;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::os::unix::fs::{FileTypeExt, PermissionsExt};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::{Value, json};

use crate::agent::orchestrator::AgentOrchestrator;
use crate::agent::tools::ToolRegistry;
use crate::config::AppConfig;
use crate::context::{ContextManager, TaskContext};
use crate::identity::ClientIdentity;
use crate::permissions::{Capability, PermissionBroker};
use crate::request_context::RequestContext;
use crate::inference::ollama::OllamaProvider;
use crate::inference::provider::{
    CancellationToken, ChatMessage, GenerationEvent, GenerationRequest, InferenceProvider,
    MessageRole,
};
use crate::storage::conversation_store::ConversationStore;
use crate::task_analyzer::{TaskAnalysis, TaskAnalyzer};

use super::protocol::{
    JsonRpcRequest, JsonRpcResponse, METHOD_CHAT_CANCEL, METHOD_CHAT_SEND,
    METHOD_CONVERSATION_CREATE, METHOD_CONVERSATION_DELETE, METHOD_CONVERSATION_GET,
    METHOD_CONVERSATION_LIST, METHOD_CONVERSATION_RENAME, METHOD_INITIALIZE, METHOD_SETTINGS_GET,
    METHOD_SETTINGS_UPDATE,
};

const MAX_FRAME_BYTES: usize = 1024 * 1024;

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

type Conversations = Arc<Mutex<HashMap<String, Vec<ChatMessage>>>>;
type Tasks = Arc<Mutex<HashMap<String, CancellationToken>>>;

struct ServerState {
    permissions: PermissionBroker,
    config: Arc<Mutex<AppConfig>>,
    provider: Arc<dyn InferenceProvider>,
    conversations: Conversations,
    store: Arc<Mutex<ConversationStore>>,
    tasks: Tasks,
    analysis_versions: Arc<Mutex<HashMap<String, u64>>>,
    tools: Arc<ToolRegistry>,
    agents: Arc<AgentRegistry>,
}

fn socket_path() -> Result<PathBuf, String> {
    let runtime_dir =
        env::var_os("XDG_RUNTIME_DIR").ok_or_else(|| "XDG_RUNTIME_DIR is not set.".to_string())?;

    let directory = PathBuf::from(runtime_dir).join("local-assistant");

    fs::create_dir_all(&directory).map_err(|e| format!("Runtime directory error: {e}"))?;

    fs::set_permissions(&directory, fs::Permissions::from_mode(0o700))
        .map_err(|e| format!("Directory permission error: {e}"))?;

    Ok(directory.join("assistantd.sock"))
}

pub fn run(config: &AppConfig) -> Result<(), String> {
    let path = socket_path()?;

    if let Ok(metadata) = fs::symlink_metadata(&path) {
        if !metadata.file_type().is_socket() {
            return Err(format!(
                "Refusing to replace non-socket file: {}",
                path.display()
            ));
        }

        match UnixStream::connect(&path) {
            Ok(_) => {
                return Err("Another IPC server is already running.".to_string());
            }
            Err(_) => {
                fs::remove_file(&path)
                    .map_err(|e| format!("Could not remove stale socket: {e}"))?;
            }
        }
    }

    let listener = UnixListener::bind(&path).map_err(|e| format!("Socket bind failed: {e}"))?;

    fs::set_permissions(&path, fs::Permissions::from_mode(0o600))
        .map_err(|e| format!("Socket permission error: {e}"))?;

    let provider = OllamaProvider::new(config.ollama_base_url.clone())?;
    let tools = Arc::new(ToolRegistry::with_builtins()?);
    let agents = Arc::new(AgentRegistry::with_builtins()?);
    let store = ConversationStore::open()?;

    let mut effective_config = config.clone();
    effective_config.context_length_tokens = store.context_length(config.context_length_tokens)?;
    effective_config.validate()?;

    let state = Arc::new(ServerState {
        permissions: PermissionBroker::for_desktop_user(unsafe { libc::geteuid() }),
        config: Arc::new(Mutex::new(effective_config.clone())),
        provider: Arc::new(provider),
        conversations: Arc::new(Mutex::new(HashMap::new())),
        store: Arc::new(Mutex::new(store)),
        tasks: Arc::new(Mutex::new(HashMap::new())),
        analysis_versions: Arc::new(Mutex::new(HashMap::new())),
        tools,
        agents: Arc::clone(&agents),
    });

    println!("Local Assistant 2.0 IPC server");
    println!("Socket: {}", path.display());
    println!("Provider: ollama");
    println!("Model: {}", effective_config.model_name);
    println!("Context: {} tokens", effective_config.context_length_tokens);
    println!("Registered agents: {}", agents.definitions().len());
    println!("Waiting for clients...");

    for incoming in listener.incoming() {
        match incoming {
            Ok(stream) => {
                let client_state = Arc::clone(&state);

                thread::spawn(move || {
                    if let Err(error) = handle_client(stream, client_state) {
                        eprintln!("IPC client error: {error}");
                    }
                });
            }
            Err(error) => eprintln!("IPC accept error: {error}"),
        }
    }

    Ok(())
}

fn handle_client(stream: UnixStream, state: Arc<ServerState>) -> Result<(), String> {
    let identity = ClientIdentity::from_stream(&stream)
        .map_err(|error| format!("Could not authenticate IPC peer: {error}"))?;

    identity
        .verify_current_user()
        .map_err(|error| format!("IPC peer rejected: {error}"))?;

    println!(
        "IPC client authenticated: pid={}, uid={}, gid={}",
        identity.pid, identity.uid, identity.gid
    );
    let reader_stream = stream.try_clone().map_err(|e| e.to_string())?;
    let mut reader = BufReader::new(reader_stream);
    let writer = Arc::new(Mutex::new(stream));

    while let Some(frame) = read_frame(&mut reader).map_err(|e| e.to_string())? {
        let request: JsonRpcRequest = match serde_json::from_slice(&frame) {
            Ok(request) => request,
            Err(error) => {
                send_response(
                    &writer,
                    &JsonRpcResponse::failure(None, -32700, format!("Parse error: {error}")),
                )?;
                continue;
            }
        };

        handle_request(request, &state, &writer, &identity)?;
    }

    Ok(())
}

fn read_frame(reader: &mut BufReader<UnixStream>) -> io::Result<Option<Vec<u8>>> {
    let mut frame = Vec::new();

    loop {
        let available = reader.fill_buf()?;

        if available.is_empty() {
            return if frame.is_empty() {
                Ok(None)
            } else {
                Ok(Some(frame))
            };
        }

        let newline = available.iter().position(|b| *b == b'\n');
        let count = newline.map_or(available.len(), |i| i + 1);

        if frame.len() + count > MAX_FRAME_BYTES {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "JSON-RPC frame exceeds 1 MiB",
            ));
        }

        frame.extend_from_slice(&available[..count]);
        reader.consume(count);

        if newline.is_some() {
            break;
        }
    }

    if frame.last() == Some(&b'\n') {
        frame.pop();
    }
    if frame.last() == Some(&b'\r') {
        frame.pop();
    }

    if frame.is_empty() {
        return Ok(Some(b"{}".to_vec()));
    }

    Ok(Some(frame))
}

fn handle_request(
    request: JsonRpcRequest,
    state: &Arc<ServerState>,
    writer: &Arc<Mutex<UnixStream>>,
    identity: &ClientIdentity,
) -> Result<(), String> {
    let id = request.id.clone();
    let notification = request.is_notification();
    let request_context = RequestContext::for_client(identity, new_id("request"));

    if let Some(reason) = request.validation_error() {
        if !notification {
            send_response(
                writer,
                &JsonRpcResponse::failure(
                    id,
                    -32600,
                    format!("Invalid Request: {reason}"),
                ),
            )?;
        }
        return Ok(());
    }

    let capability = match PermissionBroker::capability_for_method(&request.method) {
        Some(capability) => capability,
        None => {
            if !notification {
                send_response(
                    writer,
                    &JsonRpcResponse::failure(
                        id,
                        -32601,
                        format!("Unknown or unauthorized method: {}", request.method),
                    ),
                )?;
            }
            return Ok(());
        }
    };

    let decision = state.permissions.authorize(identity, capability);

    if !decision.allowed {
        if !notification {
            send_response(
                writer,
                &JsonRpcResponse::failure(
                    id,
                    -32001,
                    format!(
                        "Permission denied for {}: {}",
                        decision.capability,
                        decision.reason
                    ),
                ),
            )?;
        }
        return Ok(());
    }

    let result = match request.method.as_str() {
        METHOD_INITIALIZE => {
            let config = state
                .config
                .lock()
                .map_err(|_| "Configuration unavailable".to_string())?;

            Ok(json!({
                "server": {
                    "name": "Local Assistant 2.0",
                    "version": env!("CARGO_PKG_VERSION")
                },
                "authenticatedClient": {
                    "uid": identity.uid
                },
                "provider": state.provider.provider_name(),
                "model": config.model_name,
                "contextLength": config.context_length_tokens,
                "transport": "unix-socket-ndjson",
                "capabilities": {
                    "streaming": true,
                    "cancellation": true,
                    "conversationManagement": true,
                    "settings": true
                }
            }))
        }

        METHOD_SETTINGS_GET => {
            let config = state
                .config
                .lock()
                .map_err(|_| "Configuration unavailable".to_string())?;

            Ok(json!({
                "contextLength": config.context_length_tokens
            }))
        }

        METHOD_SETTINGS_UPDATE => {
            let value = request
                .params
                .get("contextLength")
                .and_then(Value::as_u64)
                .ok_or_else(|| "settings.update requires a numeric contextLength".to_string())?
                as usize;

            if !(4096..=16384).contains(&value) || value % 1024 != 0 {
                return Err(
                    "Context length must be 4096–16384 tokens in 1024-token steps".to_string(),
                );
            }

            state
                .store
                .lock()
                .map_err(|_| "Database unavailable".to_string())?
                .set_context_length(value)?;

            let mut config = state
                .config
                .lock()
                .map_err(|_| "Configuration unavailable".to_string())?;
            config.context_length_tokens = value;

            Ok(json!({ "contextLength": value, "saved": true }))
        }

        METHOD_CONVERSATION_CREATE => {
            let conversation_id = new_id("conversation");

            state
                .store
                .lock()
                .map_err(|_| "Database unavailable".to_string())?
                .create(&conversation_id)?;

            state
                .conversations
                .lock()
                .map_err(|_| "Conversation cache unavailable".to_string())?
                .insert(conversation_id.clone(), Vec::new());

            Ok(json!({ "conversationId": conversation_id }))
        }

        METHOD_CONVERSATION_LIST => {
            let conversations = state
                .store
                .lock()
                .map_err(|_| "Database unavailable".to_string())?
                .list()?;

            let items: Vec<Value> = conversations
                .into_iter()
                .map(|item| {
                    json!({
                        "id": item.id,
                        "title": item.title,
                        "createdAt": item.created_at,
                        "updatedAt": item.updated_at
                    })
                })
                .collect();

            Ok(json!({ "conversations": items }))
        }

        METHOD_CONVERSATION_GET => {
            let conversation_id = request
                .params
                .get("conversationId")
                .and_then(Value::as_str)
                .ok_or_else(|| "conversation.get requires conversationId".to_string())?;

            let loaded = state
                .store
                .lock()
                .map_err(|_| "Database unavailable".to_string())?
                .load(conversation_id)?;

            match loaded {
                Some((conversation, messages)) => Ok(json!({
                    "conversation": {
                        "id": conversation.id,
                        "title": conversation.title,
                        "createdAt": conversation.created_at,
                        "updatedAt": conversation.updated_at
                    },
                    "messages": messages.into_iter().map(|message| json!({
                        "id": message.id,
                        "role": message.role,
                        "content": message.content,
                        "createdAt": message.created_at
                    })).collect::<Vec<_>>()
                })),
                None => Err("Conversation not found".to_string()),
            }
        }

        METHOD_CONVERSATION_RENAME => {
            let conversation_id = request
                .params
                .get("conversationId")
                .and_then(Value::as_str)
                .ok_or_else(|| "conversation.rename requires conversationId".to_string())?;

            let title = request
                .params
                .get("title")
                .and_then(Value::as_str)
                .ok_or_else(|| "conversation.rename requires title".to_string())?;

            let renamed = state
                .store
                .lock()
                .map_err(|_| "Database unavailable".to_string())?
                .rename(conversation_id, title)?;

            if !renamed {
                return Err("Conversation not found".to_string());
            }

            Ok(json!({ "renamed": true, "conversationId": conversation_id }))
        }

        METHOD_CONVERSATION_DELETE => {
            let conversation_id = request
                .params
                .get("conversationId")
                .and_then(Value::as_str)
                .ok_or_else(|| "conversation.delete requires conversationId".to_string())?;

            let deleted = state
                .store
                .lock()
                .map_err(|_| "Database unavailable".to_string())?
                .delete(conversation_id)?;

            if !deleted {
                return Err("Conversation not found".to_string());
            }

            state
                .conversations
                .lock()
                .map_err(|_| "Conversation cache unavailable".to_string())?
                .remove(conversation_id);

            Ok(json!({ "deleted": true, "conversationId": conversation_id }))
        }

        METHOD_CHAT_SEND => {
            if notification {
                return Ok(());
            }

            let conversation_id = request
                .params
                .get("conversationId")
                .and_then(Value::as_str)
                .ok_or_else(|| "chat.send requires conversationId".to_string());

            let text = request
                .params
                .get("text")
                .and_then(Value::as_str)
                .filter(|text| !text.trim().is_empty())
                .ok_or_else(|| "chat.send requires non-empty text".to_string());

            match (conversation_id, text) {
                (Ok(conversation_id), Ok(text)) => {
                    let task_context = TaskContext::from_value(request.params.get("taskContext"))?;

                    let tool_authorized = state
                        .permissions
                        .authorize(identity, Capability::AgentToolExecute)
                        .allowed;

                    start_generation(
                        conversation_id.to_string(),
                        text.to_string(),
                        task_context,
                        tool_authorized,
                        request_context.clone(),
                        state,
                        writer,
                    )
                }
                (Err(error), _) | (_, Err(error)) => Err(error),
            }
        }

        METHOD_CHAT_CANCEL => {
            let task_id = request
                .params
                .get("taskId")
                .and_then(Value::as_str)
                .ok_or_else(|| "chat.cancel requires taskId".to_string());

            match task_id {
                Ok(task_id) => {
                    let tasks = state
                        .tasks
                        .lock()
                        .map_err(|_| "Task registry unavailable".to_string())?;

                    match tasks.get(task_id) {
                        Some(token) => {
                            token.cancel();
                            Ok(json!({ "cancelled": true, "taskId": task_id }))
                        }
                        None => Err("Task not found or already completed".to_string()),
                    }
                }
                Err(error) => Err(error),
            }
        }

        _ => {
            if notification {
                return Ok(());
            }
            Err(format!("Method not found: {}", request.method))
        }
    };

    if notification {
        return Ok(());
    }

    let response = match result {
        Ok(value) => JsonRpcResponse::success(id, value),
        Err(message) => JsonRpcResponse::failure(id, -32602, message),
    };

    send_response(writer, &response)
}

fn start_generation(
    conversation_id: String,
    user_text: String,
    task_context: Option<TaskContext>,
    tool_authorized: bool,
    request_context: RequestContext,
    state: &Arc<ServerState>,
    writer: &Arc<Mutex<UnixStream>>,
) -> Result<Value, String> {
    let (history, task_context) = {
        let store = state
            .store
            .lock()
            .map_err(|_| "Database unavailable".to_string())?;

        let (_, stored_messages) = store
            .load(&conversation_id)?
            .ok_or_else(|| "Conversation not found".to_string())?;

        // Explicitly supplied context replaces the saved context.
        // Otherwise, reuse the context already associated with this conversation.
        let effective_task_context = match task_context {
            Some(context) => Some(context),
            None => match store.load_task_context(&conversation_id)? {
                Some(value) => TaskContext::from_value(Some(&value))?,
                None => None,
            },
        };

        let mut history = Vec::new();

        for message in stored_messages {
            let role = match message.role.as_str() {
                "user" => MessageRole::User,
                "assistant" => MessageRole::Assistant,
                _ => continue,
            };

            history.push(ChatMessage {
                role,
                content: message.content,
            });
        }

        (history, effective_task_context)
    };

    let task_id = new_id("task");
    let request_context = request_context.with_task_id(task_id.clone());
    request_context.log_event("generation_started", None);

    // Each new generation invalidates background analysis from older turns
    // in the same conversation.
    let analysis_version = {
        let mut versions = state
            .analysis_versions
            .lock()
            .map_err(|_| "Task analysis registry unavailable".to_string())?;

        let version = versions.entry(conversation_id.clone()).or_insert(0);
        *version = version.saturating_add(1);
        *version
    };

    let cancellation = CancellationToken::new();

    state
        .tasks
        .lock()
        .map_err(|_| "Task registry unavailable".to_string())?
        .insert(task_id.clone(), cancellation.clone());

    let provider = Arc::clone(&state.provider);
    let tools = Arc::clone(&state.tools);
    let agents = Arc::clone(&state.agents);
    let conversations = Arc::clone(&state.conversations);
    let store = Arc::clone(&state.store);
    let tasks = Arc::clone(&state.tasks);
    let analysis_versions = Arc::clone(&state.analysis_versions);
    let writer = Arc::clone(writer);
    let (model, context_length_tokens) = {
        let config = state
            .config
            .lock()
            .map_err(|_| "Configuration unavailable".to_string())?;

        (config.model_name.clone(), config.context_length_tokens)
    };

    let messages = ContextManager::build_messages(
        &history,
        &user_text,
        context_length_tokens,
        task_context.as_ref(),
    );

    let response_conversation_id = conversation_id.clone();
    let response_task_id = task_id.clone();
    let response_request_id = request_context.request_id.clone();

    thread::spawn(move || {
        let analysis_model = model.clone();

        let agent_context = if tool_authorized {
            match agents.create_context("general", &request_context) {
                Ok(context) => Some(context),
                Err(error) => {
                    eprintln!("[agent] Could not create agent context: {error}");
                    None
                }
            }
        } else {
            None
        };

        let messages = AgentOrchestrator::prepare_messages(
            provider.as_ref(),
            &model,
            messages,
            context_length_tokens,
            agent_context,
            tools.as_ref(),
        );

        let request = GenerationRequest {
            model,
            messages,
            context_length_tokens,
        };

        let (sender, receiver) = mpsc::channel();
        let worker_provider = Arc::clone(&provider);
        let worker_cancellation = cancellation.clone();

        let worker =
            thread::spawn(move || worker_provider.generate(request, sender, worker_cancellation));

        let mut response_text = String::new();
        let mut completion_received = false;
        let mut cancellation_received = false;
        let mut failure_received: Option<String> = None;
        let mut terminal_event = false;

        for event in receiver {
            match event {
                GenerationEvent::TextDelta(delta) => {
                    if terminal_event {
                        continue;
                    }

                    response_text.push_str(&delta);

                    let notification = json!({
                        "jsonrpc": "2.0",
                        "method": "chat.delta",
                        "params": {
                            "conversationId": conversation_id,
                            "taskId": task_id,
                            "requestId": request_context.request_id.clone(),
                            "delta": delta
                        }
                    });

                    let _ = send_json(&writer, &notification);
                }
                GenerationEvent::Completed => {
                    if !terminal_event {
                        terminal_event = true;
                        completion_received = true;
                    }
                }
                GenerationEvent::Cancelled => {
                    if !terminal_event {
                        terminal_event = true;
                        cancellation_received = true;
                    }
                }
                GenerationEvent::Failed(error) => {
                    if !terminal_event {
                        terminal_event = true;
                        failure_received = Some(error);
                    }
                }
            }
        }

        let worker_result = worker.join();

        let terminal_notification = match worker_result {
            Err(_) => {
                request_context.log_event(
                    "generation_failed",
                    Some("Inference worker panicked"),
                );
                json!({
                "jsonrpc": "2.0",
                "method": "chat.failed",
                "params": {
                    "conversationId": conversation_id,
                    "taskId": task_id,
                    "requestId": request_context.request_id.clone(),
                    "error": "Inference worker panicked"
                }
            })
            },

            Ok(Err(error)) => {
                request_context.log_event("generation_failed", Some(&error));
                json!({
                "jsonrpc": "2.0",
                "method": "chat.failed",
                "params": {
                    "conversationId": conversation_id,
                    "taskId": task_id,
                    "requestId": request_context.request_id.clone(),
                    "error": failure_received.unwrap_or(error)
                }
            })
            },

            Ok(Ok(())) if failure_received.is_some() => {
                let error = failure_received.unwrap();
                request_context.log_event("generation_failed", Some(&error));
                json!({
                "jsonrpc": "2.0",
                "method": "chat.failed",
                "params": {
                    "conversationId": conversation_id,
                    "taskId": task_id,
                    "requestId": request_context.request_id.clone(),
                    "error": error
                }
            })
            },

            Ok(Ok(())) if cancellation_received => {
                request_context.log_event("generation_cancelled", None);
                json!({
                "jsonrpc": "2.0",
                "method": "chat.cancelled",
                "params": {
                    "conversationId": conversation_id,
                    "taskId": task_id,
                    "requestId": request_context.request_id.clone()
                }
            })
            },

            Ok(Ok(())) if completion_received => {
                let saved = store
                    .lock()
                    .map_err(|_| "Database unavailable".to_string())
                    .and_then(|mut database| {
                        database.append_exchange(&conversation_id, &user_text, &response_text)?;

                        // Explicit context is committed only when its exchange
                        // has also been saved successfully.
                        if let Some(context) = task_context.as_ref() {
                            let value = json!({
                                "objective": context.objective,
                                "constraints": context.constraints,
                                "additionalContext": context.additional_context
                            });

                            if let Err(error) =
                                database.save_task_context(&conversation_id, &value)
                            {
                                eprintln!(
                                    "Conversation saved, but explicit task context could not be saved: {error}"
                                );
                            }
                        }

                        Ok(())
                    });

                match saved {
                    Ok(()) => {
                        request_context.log_event("generation_completed", None);

                        if let Ok(mut conversations) = conversations.lock() {
                            if let Some(history) = conversations.get_mut(&conversation_id) {
                                history.push(ChatMessage {
                                    role: MessageRole::User,
                                    content: user_text.clone(),
                                });
                                history.push(ChatMessage {
                                    role: MessageRole::Assistant,
                                    content: response_text.clone(),
                                });
                            }
                        }

                        // Task analysis runs independently after the user's
                        // response has been saved. Its failure never changes
                        // the successful chat result.
                        let analysis_provider = Arc::clone(&provider);
                        let analysis_store = Arc::clone(&store);
                        let analysis_versions = Arc::clone(&analysis_versions);
                        let analysis_conversation_id = conversation_id.clone();
                        let analysis_user_text = user_text.clone();
                        let analysis_response_text = response_text.clone();
                        let analysis_existing_context = task_context.clone();
                        let analysis_model = analysis_model.clone();
                        let analysis_request_context = request_context.clone();

                        thread::spawn(move || {
                            // Check whether this analysis is already obsolete.
                            // Release the lock before performing model inference.
                            let is_current = match analysis_versions.lock() {
                                Ok(versions) => {
                                    versions.get(&analysis_conversation_id).copied()
                                        == Some(analysis_version)
                                }
                                Err(_) => {
                                    analysis_request_context.log_event(
                                        "task_analysis_registry_unavailable", None,
                                    );
                                    return;
                                }
                            };

                            if !is_current {
                                analysis_request_context.log_event(
                                    "task_analysis_stale", None,
                                );
                                return;
                            }

                            analysis_request_context.log_event(
                                "task_analysis_started", None,
                            );

                            let analysis = match TaskAnalyzer::analyze(
                                analysis_provider.as_ref(),
                                &analysis_model,
                                analysis_existing_context.as_ref(),
                                &analysis_user_text,
                                &analysis_response_text,
                            ) {
                                Ok(result) => result,
                                Err(error) => {
                                    analysis_request_context.log_event(
                                        "task_analysis_failed", Some(&error),
                                    );
                                    return;
                                }
                            };

                            if matches!(analysis, TaskAnalysis::NoChange) {
                                analysis_request_context.log_event(
                                    "task_analysis_no_change", None,
                                );
                                return;
                            }

                            // Recheck after inference. Keep this lock through the
                            // database write so a newer generation cannot start
                            // between the version check and the context update.
                            let versions = match analysis_versions.lock() {
                                Ok(versions) => versions,
                                Err(_) => {
                                    analysis_request_context.log_event(
                                        "task_analysis_registry_unavailable", None,
                                    );
                                    return;
                                }
                            };

                            if versions.get(&analysis_conversation_id).copied()
                                != Some(analysis_version)
                            {
                                analysis_request_context.log_event(
                                    "task_analysis_stale", None,
                                );
                                return;
                            }

                            let mut database = match analysis_store.lock() {
                                Ok(database) => database,
                                Err(_) => {
                                    analysis_request_context.log_event(
                                        "task_context_database_unavailable", None,
                                    );
                                    return;
                                }
                            };

                            match analysis {
                                TaskAnalysis::NoChange => {}
                                TaskAnalysis::Set(context) => {
                                    let value = json!({
                                        "objective": context.objective,
                                        "constraints": context.constraints,
                                        "additionalContext": context.additional_context
                                    });

                                    if let Err(error) = database.save_task_context(
                                        &analysis_conversation_id,
                                        &value,
                                    ) {
                                        analysis_request_context.log_event(
                                            "task_context_save_failed", Some(&error),
                                        );
                                    } else {
                                        analysis_request_context.log_event(
                                            "task_context_updated", None,
                                        );
                                        analysis_request_context.log_event(
                                            "task_analysis_completed", None,
                                        );
                                    }
                                }
                                TaskAnalysis::Clear => {
                                    if let Err(error) =
                                        database.clear_task_context(&analysis_conversation_id)
                                    {
                                        analysis_request_context.log_event(
                                            "task_context_clear_failed", Some(&error),
                                        );
                                    } else {
                                        analysis_request_context.log_event(
                                            "task_context_cleared", None,
                                        );
                                        analysis_request_context.log_event(
                                            "task_analysis_completed", None,
                                        );
                                    }
                                }
                            }
                        });

                        json!({
                            "jsonrpc": "2.0",
                            "method": "chat.completed",
                            "params": {
                                "conversationId": conversation_id,
                                "taskId": task_id,
                                "requestId": request_context.request_id.clone()
                            }
                        })
                    }
                    Err(error) => {
                        request_context.log_event("generation_save_failed", Some(&error));
                        json!({
                            "jsonrpc": "2.0",
                            "method": "chat.failed",
                            "params": {
                                "conversationId": conversation_id,
                                "taskId": task_id,
                                "requestId": request_context.request_id.clone(),
                                "error": format!(
                                    "Response generated but could not be saved: {error}"
                                )
                            }
                        })
                    },
                }
            }

            Ok(Ok(())) => {
                request_context.log_event(
                    "generation_failed",
                    Some("Generation ended without a completion event"),
                );
                json!({
                    "jsonrpc": "2.0",
                    "method": "chat.failed",
                    "params": {
                        "conversationId": conversation_id,
                        "taskId": task_id,
                        "requestId": request_context.request_id.clone(),
                        "error": "Generation ended without a completion event"
                    }
                })
            },
        };

        let _ = send_json(&writer, &terminal_notification);

        if let Ok(mut tasks) = tasks.lock() {
            tasks.remove(&task_id);
        }
    });

    Ok(json!({
        "accepted": true,
        "conversationId": response_conversation_id,
        "taskId": response_task_id,
        "requestId": response_request_id
    }))
}

fn new_id(prefix: &str) -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();

    let sequence = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    format!("{prefix}-{timestamp:x}-{sequence:x}")
}

fn send_response(
    writer: &Arc<Mutex<UnixStream>>,
    response: &JsonRpcResponse,
) -> Result<(), String> {
    let value =
        serde_json::to_value(response).map_err(|e| format!("Response encoding error: {e}"))?;
    send_json(writer, &value)
}

fn send_json(writer: &Arc<Mutex<UnixStream>>, value: &Value) -> Result<(), String> {
    let encoded = serde_json::to_vec(value).map_err(|e| format!("JSON encoding error: {e}"))?;

    let mut stream = writer
        .lock()
        .map_err(|_| "Client connection unavailable".to_string())?;

    stream.write_all(&encoded).map_err(|e| e.to_string())?;
    stream.write_all(b"\n").map_err(|e| e.to_string())?;
    stream.flush().map_err(|e| e.to_string())
}
