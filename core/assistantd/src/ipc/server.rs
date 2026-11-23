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

use crate::config::AppConfig;
use crate::inference::ollama::OllamaProvider;
use crate::inference::provider::{
    CancellationToken, ChatMessage, GenerationEvent, GenerationRequest, InferenceProvider,
    MessageRole,
};
use crate::storage::conversation_store::ConversationStore;

use super::protocol::{
    JSON_RPC_VERSION, JsonRpcRequest, JsonRpcResponse, METHOD_CHAT_CANCEL, METHOD_CHAT_SEND,
    METHOD_CONVERSATION_CREATE, METHOD_CONVERSATION_DELETE, METHOD_CONVERSATION_GET,
    METHOD_CONVERSATION_LIST, METHOD_CONVERSATION_RENAME, METHOD_INITIALIZE, METHOD_SETTINGS_GET,
    METHOD_SETTINGS_UPDATE,
};

const MAX_FRAME_BYTES: usize = 1024 * 1024;

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

type Conversations = Arc<Mutex<HashMap<String, Vec<ChatMessage>>>>;
type Tasks = Arc<Mutex<HashMap<String, CancellationToken>>>;

struct ServerState {
    config: Arc<Mutex<AppConfig>>,
    provider: Arc<dyn InferenceProvider>,
    conversations: Conversations,
    store: Arc<Mutex<ConversationStore>>,
    tasks: Tasks,
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
    let store = ConversationStore::open()?;

    let mut effective_config = config.clone();
    effective_config.context_length_tokens = store.context_length(config.context_length_tokens)?;
    effective_config.validate()?;

    let state = Arc::new(ServerState {
        config: Arc::new(Mutex::new(effective_config.clone())),
        provider: Arc::new(provider),
        conversations: Arc::new(Mutex::new(HashMap::new())),
        store: Arc::new(Mutex::new(store)),
        tasks: Arc::new(Mutex::new(HashMap::new())),
    });

    println!("Local Assistant 2.0 IPC server");
    println!("Socket: {}", path.display());
    println!("Provider: ollama");
    println!("Model: {}", effective_config.model_name);
    println!("Context: {} tokens", effective_config.context_length_tokens);
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

        handle_request(request, &state, &writer)?;
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
) -> Result<(), String> {
    let id = request.id.clone();
    let notification = request.is_notification();

    if request.jsonrpc != JSON_RPC_VERSION {
        if !notification {
            send_response(
                writer,
                &JsonRpcResponse::failure(id, -32600, "Invalid Request: jsonrpc must be 2.0"),
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
                .insert(conversation_id.clone(), vec![system_message()]);

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
                    start_generation(conversation_id.to_string(), text.to_string(), state, writer)
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
    state: &Arc<ServerState>,
    writer: &Arc<Mutex<UnixStream>>,
) -> Result<Value, String> {
    let history = {
        let store = state
            .store
            .lock()
            .map_err(|_| "Database unavailable".to_string())?;

        let (_, stored_messages) = store
            .load(&conversation_id)?
            .ok_or_else(|| "Conversation not found".to_string())?;

        let mut history = vec![system_message()];

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

        history
    };

    let task_id = new_id("task");
    let cancellation = CancellationToken::new();

    state
        .tasks
        .lock()
        .map_err(|_| "Task registry unavailable".to_string())?
        .insert(task_id.clone(), cancellation.clone());

    let provider = Arc::clone(&state.provider);
    let conversations = Arc::clone(&state.conversations);
    let store = Arc::clone(&state.store);
    let tasks = Arc::clone(&state.tasks);
    let writer = Arc::clone(writer);
    let (model, context_length_tokens) = {
        let config = state
            .config
            .lock()
            .map_err(|_| "Configuration unavailable".to_string())?;

        (config.model_name.clone(), config.context_length_tokens)
    };
    let response_conversation_id = conversation_id.clone();
    let response_task_id = task_id.clone();

    thread::spawn(move || {
        let mut messages = history;
        messages.push(ChatMessage {
            role: MessageRole::User,
            content: user_text.clone(),
        });

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
        let mut completed = false;
        let mut terminal_event = false;

        for event in receiver {
            let notification = match event {
                GenerationEvent::TextDelta(delta) => {
                    response_text.push_str(&delta);
                    json!({
                        "jsonrpc": JSON_RPC_VERSION,
                        "method": "chat.delta",
                        "params": {
                            "conversationId": conversation_id,
                            "taskId": task_id,
                            "delta": delta
                        }
                    })
                }
                GenerationEvent::Completed => {
                    completed = true;
                    terminal_event = true;
                    json!({
                        "jsonrpc": JSON_RPC_VERSION,
                        "method": "chat.completed",
                        "params": {
                            "conversationId": conversation_id,
                            "taskId": task_id
                        }
                    })
                }
                GenerationEvent::Cancelled => {
                    terminal_event = true;
                    json!({
                        "jsonrpc": JSON_RPC_VERSION,
                        "method": "chat.cancelled",
                        "params": {
                            "conversationId": conversation_id,
                            "taskId": task_id
                        }
                    })
                }
                GenerationEvent::Failed(error) => {
                    terminal_event = true;
                    json!({
                        "jsonrpc": JSON_RPC_VERSION,
                        "method": "chat.failed",
                        "params": {
                            "conversationId": conversation_id,
                            "taskId": task_id,
                            "error": error
                        }
                    })
                }
            };

            let _ = send_json(&writer, &notification);
        }

        let worker_result = worker.join();

        if completed {
            let saved = store
                .lock()
                .map_err(|_| "Database unavailable".to_string())
                .and_then(|mut database| {
                    database.append_exchange(&conversation_id, &user_text, &response_text)
                });

            match saved {
                Ok(()) => {
                    if let Ok(mut conversations) = conversations.lock() {
                        if let Some(history) = conversations.get_mut(&conversation_id) {
                            history.push(ChatMessage {
                                role: MessageRole::User,
                                content: user_text,
                            });
                            history.push(ChatMessage {
                                role: MessageRole::Assistant,
                                content: response_text,
                            });
                        }
                    }
                }
                Err(error) => {
                    let _ = send_json(
                        &writer,
                        &json!({
                            "jsonrpc": JSON_RPC_VERSION,
                            "method": "chat.failed",
                            "params": {
                                "conversationId": conversation_id,
                                "taskId": task_id,
                                "error": format!("Response generated but could not be saved: {error}")
                            }
                        }),
                    );
                }
            }
        } else if !terminal_event {
            let message = match worker_result {
                Ok(Err(error)) => error,
                Err(_) => "Inference worker panicked".to_string(),
                Ok(Ok(())) => "Generation ended without a completion event".to_string(),
            };

            let _ = send_json(
                &writer,
                &json!({
                    "jsonrpc": JSON_RPC_VERSION,
                    "method": "chat.failed",
                    "params": {
                        "conversationId": conversation_id,
                        "taskId": task_id,
                        "error": message
                    }
                }),
            );
        }

        if let Ok(mut tasks) = tasks.lock() {
            tasks.remove(&task_id);
        }
    });

    Ok(json!({
        "accepted": true,
        "conversationId": response_conversation_id,
        "taskId": response_task_id
    }))
}

fn system_message() -> ChatMessage {
    ChatMessage {
        role: MessageRole::System,
        content: concat!(
            "You are Local Assistant 2.0, a friendly, intelligent, ",
            "and capable desktop AI assistant. ",
            "Your personality is warm, calm, relaxed, and approachable. ",
            "Sound natural and conversational, like a thoughtful person ",
            "having a genuine everyday conversation. ",
            "Do not sound robotic, overly formal, or excessively enthusiastic. ",
            "Avoid forced excitement, exaggerated praise, and unnecessary ",
            "compliments. ",
            "Use emojis sparingly and only when they fit naturally. ",
            "Be supportive without overdoing it. ",
            "Give explanations with enough detail to be genuinely useful. ",
            "Adapt response length to the user's request. ",
            "Use conversation history to maintain context. ",
            "Avoid unnecessary personal questions. ",
            "Never pressure the user to continue chatting. ",
            "Be honest about limitations and actual capabilities. ",
            "Never claim planned features are already implemented."
        )
        .to_string(),
    }
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
