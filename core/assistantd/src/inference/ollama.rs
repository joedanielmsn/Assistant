use std::io::{BufRead, BufReader};
use std::sync::mpsc::Sender;

use reqwest::blocking::Client;
use serde_json::{Value, json};

use super::provider::{
    CancellationToken, ChatMessage, GenerationEvent, GenerationRequest, InferenceProvider,
    MessageRole,
};

pub struct OllamaProvider {
    base_url: String,
    client: Client,
}

impl OllamaProvider {
    pub fn new(base_url: impl Into<String>) -> Result<Self, String> {
        let client = Client::builder()
            .build()
            .map_err(|error| format!("Failed to create HTTP client: {error}"))?;

        Ok(Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            client,
        })
    }

    fn role_name(role: &MessageRole) -> &'static str {
        match role {
            MessageRole::System => "system",
            MessageRole::User => "user",
            MessageRole::Assistant => "assistant",
        }
    }

    fn send_failure(events: &Sender<GenerationEvent>, message: String) -> Result<(), String> {
        let _ = events.send(GenerationEvent::Failed(message.clone()));
        Err(message)
    }
}

impl InferenceProvider for OllamaProvider {
    fn provider_name(&self) -> &'static str {
        "ollama"
    }

    fn generate(
        &self,
        request: GenerationRequest,
        events: Sender<GenerationEvent>,
        cancellation: CancellationToken,
    ) -> Result<(), String> {
        let messages: Vec<Value> = request
            .messages
            .iter()
            .map(|message: &ChatMessage| {
                json!({
                    "role": Self::role_name(&message.role),
                    "content": message.content
                })
            })
            .collect();

        let payload = json!({
            "model": request.model,
            "messages": messages,
            "stream": true,
            "think": false,
            "options": {
                "num_ctx": request.context_length_tokens
            }
        });

        let response = match self
            .client
            .post(format!("{}/api/chat", self.base_url))
            .json(&payload)
            .send()
            .and_then(|response| response.error_for_status())
        {
            Ok(response) => response,
            Err(error) => {
                return Self::send_failure(&events, format!("Ollama request failed: {error}"));
            }
        };

        let reader = BufReader::new(response);

        for line in reader.lines() {
            if cancellation.is_cancelled() {
                let _ = events.send(GenerationEvent::Cancelled);
                return Ok(());
            }

            let line = match line {
                Ok(line) => line,
                Err(error) => {
                    return Self::send_failure(
                        &events,
                        format!("Failed to read Ollama stream: {error}"),
                    );
                }
            };

            if line.trim().is_empty() {
                continue;
            }

            let chunk: Value = match serde_json::from_str(&line) {
                Ok(chunk) => chunk,
                Err(error) => {
                    return Self::send_failure(
                        &events,
                        format!("Invalid Ollama response: {error}"),
                    );
                }
            };

            if let Some(error) = chunk.get("error").and_then(Value::as_str) {
                return Self::send_failure(&events, format!("Ollama error: {error}"));
            }

            if let Some(content) = chunk
                .get("message")
                .and_then(|message| message.get("content"))
                .and_then(Value::as_str)
                && !content.is_empty()
            {
                if events
                    .send(GenerationEvent::TextDelta(content.to_string()))
                    .is_err()
                {
                    return Err("Generation receiver disconnected.".to_string());
                }
            }

            if chunk.get("done").and_then(Value::as_bool) == Some(true) {
                let _ = events.send(GenerationEvent::Completed);
                return Ok(());
            }
        }

        if cancellation.is_cancelled() {
            let _ = events.send(GenerationEvent::Cancelled);
        } else {
            let _ = events.send(GenerationEvent::Completed);
        }

        Ok(())
    }

    fn generate_json(&self, request: GenerationRequest) -> Result<Value, String> {
        let messages: Vec<Value> = request
            .messages
            .iter()
            .map(|message| {
                json!({
                    "role": Self::role_name(&message.role),
                    "content": message.content
                })
            })
            .collect();

        let payload = json!({
            "model": request.model,
            "messages": messages,
            "stream": false,
            "think": false,
            "format": "json",
            "options": {
                "num_ctx": request.context_length_tokens,
                "temperature": 0.1
            }
        });

        let response = self
            .client
            .post(format!("{}/api/chat", self.base_url))
            .json(&payload)
            .send()
            .and_then(|response| response.error_for_status())
            .map_err(|error| format!("Ollama JSON request failed: {error}"))?;

        let body: Value = response
            .json()
            .map_err(|error| format!("Invalid Ollama JSON response: {error}"))?;

        if let Some(error) = body.get("error").and_then(Value::as_str) {
            return Err(format!("Ollama error: {error}"));
        }

        let content = body
            .get("message")
            .and_then(|message| message.get("content"))
            .and_then(Value::as_str)
            .ok_or_else(|| "Ollama returned no JSON message content".to_string())?;

        serde_json::from_str(content)
            .map_err(|error| format!("Task analysis returned invalid JSON: {error}"))
    }
}
