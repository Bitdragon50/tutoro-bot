use ollama_rs::{error::OllamaError, Ollama};
use ollama_rs::generation::completion::request::GenerationRequest;



pub async fn generate( prompt: &str) -> Result<String, OllamaError> {
// By default, it will connect to localhost:11434
    let ollama = Ollama::default();
    let model = "llama3-chatqa:latest".to_string();
    let prompt = prompt.to_string();    
    let response = ollama.generate(GenerationRequest::new(model, prompt)).await;

    match response {
        Ok(generation) => Ok(generation.response),
        Err(e) => {
            let err = &e.to_string();
            eprintln!("Error: {}", err);
            Err(e)
        }
    }
}