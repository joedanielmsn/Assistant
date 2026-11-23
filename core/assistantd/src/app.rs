use std::io::{self, Write};
use std::sync::{Arc, mpsc};
use std::thread;

use crate::config::AppConfig;
use crate::inference::ollama::OllamaProvider;
use crate::inference::provider::{
    CancellationToken,
    ChatMessage,
    GenerationEvent,
    GenerationRequest,
    InferenceProvider,
    MessageRole,
};

pub struct Application {
    config: AppConfig,
}

impl Application {
    pub fn new(config: AppConfig) -> Result<Self, String> {
        config.validate()?;
        Ok(Self { config })
    }

    pub fn run(&self) {
        println!("Local Assistant 2.0");
        println!("Core: assistantd");
        println!("Context length: {} tokens", self.config.context_length_tokens);

        let provider = match OllamaProvider::new(
            self.config.ollama_base_url.clone(),
        ) {
            Ok(provider) => Arc::new(provider),
            Err(error) => {
                eprintln!("Provider initialization failed: {error}");
                return;
            }
        };

        println!("Provider: {}", provider.provider_name());
        println!("Model: {}", self.config.model_name);
        println!("Type 'exit' or 'quit' to leave.");
        println!();

        let mut history = vec![
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
                    "Do not make every interaction sound like a special occasion. ",
                    "Use emojis sparingly and only when they fit naturally. ",
                    "Respond to greetings, jokes, compliments, and casual conversation ",
                    "in a relaxed and genuine-sounding way. ",
                    "Be supportive and encouraging without overdoing it. ",
                    "Give explanations with enough detail to be genuinely useful. ",
                    "Do not make responses unnecessarily short or excessively long. ",
                    "Adapt your response length to the user's request. ",
                    "Use conversation history to maintain context and continuity. ",
                    "Avoid repeatedly using the user's name or asking unnecessary ",
                    "personal questions. ",
                    "Respect conversation endings with a simple, friendly goodbye. ",
                    "Never pressure the user to continue chatting. ",
                    "Be honest about your limitations and actual available capabilities. ",
                    "Never claim that planned features are already implemented. ",
                    "Ask follow-up questions naturally when they are genuinely necessary."
                ).to_string(),
            }
        ];

        let stdin = io::stdin();

        loop {
            print!("You: ");
            let _ = io::stdout().flush();

            let mut input = String::new();

            match stdin.read_line(&mut input) {
                Ok(0) => break,
                Ok(_) => {}
                Err(error) => {
                    eprintln!("Input error: {error}");
                    break;
                }
            }

            let input = input.trim();

            if input.is_empty() {
                continue;
            }

            if input.eq_ignore_ascii_case("exit")
                || input.eq_ignore_ascii_case("quit")
            {
                println!("Goodbye.");
                break;
            }

            history.push(ChatMessage {
                role: MessageRole::User,
                content: input.to_string(),
            });

            let request = GenerationRequest {
                model: self.config.model_name.clone(),
                messages: history.clone(),
                context_length_tokens: self.config.context_length_tokens,
            };

            let (sender, receiver) = mpsc::channel();
            let cancellation = CancellationToken::new();

            let worker_provider = Arc::clone(&provider);

            let worker = thread::spawn(move || {
                worker_provider.generate(request, sender, cancellation)
            });

            print!("Assistant: ");
            let _ = io::stdout().flush();

            let mut response = String::new();

            for event in receiver {
                match event {
                    GenerationEvent::TextDelta(text) => {
                        print!("{text}");
                        let _ = io::stdout().flush();
                        response.push_str(&text);
                    }
                    GenerationEvent::Completed => {
                        println!();
                    }
                    GenerationEvent::Cancelled => {
                        println!("\nGeneration cancelled.");
                    }
                    GenerationEvent::Failed(error) => {
                        eprintln!("\nInference error: {error}");
                    }
                }
            }

            match worker.join() {
                Ok(Ok(())) => {
                    if !response.is_empty() {
                        history.push(ChatMessage {
                            role: MessageRole::Assistant,
                            content: response,
                        });
                    }
                }
                Ok(Err(error)) => {
                    eprintln!("Generation failed: {error}");
                    history.pop();
                }
                Err(_) => {
                    eprintln!("Inference worker panicked.");
                    history.pop();
                }
            }

            println!();
        }
    }
}
