use serenity::all::{Context, Message};
use crate::{ollama::{self}, vertex::call_vertex_client};
pub struct Tutoro {
    pub brain: Llm,
    context: Context,
    message: Message
}

impl Tutoro {
    pub async fn respond_to_message(&self) {
        let response = self.generate_response().await;
        match self.send_response_to_message(response).await {
            Ok(_) => println!("Message sent successfully"),
            Err(why) => println!("Error sending message: {:?}", why)
        }
    }
    pub async fn generate_response(&self) -> String {
        let prompt = format!("You are Tutoro, a helpful academic tutor. You love to help students learn to how to solve their problems and learn more about the world. Be very concise. Respond to the following message: {}", self.message.content);
        let response = self.brain.generate(prompt).await;//generate(&prompt).await;//
        match response {
            Ok(response) => response,
            Err(e) => {
                format!("Sorry, I couldn't generate a response. I encountered this Error {}", &e.to_string())
            }
        }
                
    }
    pub fn new(llm: &str, context: Context, message: Message) -> Self {
        Tutoro { 
            brain: Llm::new(llm), // or "ollama" based on your choice
            context, 
            message 
        }
    }
    async fn send_response_to_message(&self, response: String ) -> Result<serenity::all::Message, serenity::Error>{
        self.message.channel_id.say(&self.context.http, response).await
    }
}

pub enum Llm {
    Ollama,
    Vertex
}

impl Llm {
    pub fn new(model: &str) -> Self {
        match model {
            "vertex" => Llm::Vertex,
            _ => Llm::Ollama
        }
    }
    pub async fn generate(&self, prompt: String) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            Llm::Ollama => ollama::generate(&prompt).await,
            Llm::Vertex => call_vertex_client(prompt).await
        }
    }
}