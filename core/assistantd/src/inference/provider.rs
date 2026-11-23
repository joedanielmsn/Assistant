use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
    mpsc::Sender,
};

#[derive(Debug, Clone)]
pub enum MessageRole {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct GenerationRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub context_length_tokens: usize,
}

#[derive(Debug, Clone)]
pub enum GenerationEvent {
    TextDelta(String),
    Completed,
    Cancelled,
    Failed(String),
}

#[derive(Debug, Clone)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    pub fn new() -> Self {
        Self {
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Relaxed);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }
}

pub trait InferenceProvider: Send + Sync {
    fn provider_name(&self) -> &'static str;

    fn generate(
        &self,
        request: GenerationRequest,
        events: Sender<GenerationEvent>,
        cancellation: CancellationToken,
    ) -> Result<(), String>;
}
