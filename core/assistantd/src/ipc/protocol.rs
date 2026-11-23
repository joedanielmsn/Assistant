use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const JSON_RPC_VERSION: &str = "2.0";

pub const METHOD_INITIALIZE: &str = "initialize";
pub const METHOD_CHAT_SEND: &str = "chat.send";
pub const METHOD_CHAT_CANCEL: &str = "chat.cancel";
pub const METHOD_CONVERSATION_CREATE: &str = "conversation.create";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub method: String,

    #[serde(default)]
    pub params: Value,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Value>,
}

impl JsonRpcRequest {
    pub fn is_notification(&self) -> bool {
        self.id.is_none()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,

    pub id: Option<Value>,
}

impl JsonRpcResponse {
    pub fn success(id: Option<Value>, result: Value) -> Self {
        Self {
            jsonrpc: JSON_RPC_VERSION.to_string(),
            result: Some(result),
            error: None,
            id,
        }
    }

    pub fn failure(
        id: Option<Value>,
        code: i64,
        message: impl Into<String>,
    ) -> Self {
        Self {
            jsonrpc: JSON_RPC_VERSION.to_string(),
            result: None,
            error: Some(JsonRpcError {
                code,
                message: message.into(),
                data: None,
            }),
            id,
        }
    }
}
