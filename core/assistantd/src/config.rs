#[derive(Debug, Clone)]
pub struct AppConfig {
    pub context_length_tokens: usize,
    pub ollama_base_url: String,
    pub model_name: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            context_length_tokens: 16384,
            ollama_base_url: "http://localhost:11434".to_string(),
            model_name: "assistant-qwen:latest".to_string(),
        }
    }
}

impl AppConfig {
    pub fn validate(&self) -> Result<(), String> {
        if !(4096..=16384).contains(&self.context_length_tokens) {
            return Err(
                "Context length must be between 4096 and 16384 tokens.".to_string()
            );
        }

        if self.ollama_base_url.trim().is_empty() {
            return Err("Ollama URL cannot be empty.".to_string());
        }

        if self.model_name.trim().is_empty() {
            return Err("Model name cannot be empty.".to_string());
        }

        Ok(())
    }
}
