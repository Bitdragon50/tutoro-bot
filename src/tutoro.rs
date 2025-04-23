use serenity::all::{Context, Message};
use crate::{ollama::generate, vertex::call_vertex_client};
pub struct Tutoro {
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
        let response = call_vertex_client(prompt).await;//generate(&prompt).await;//
        match response {
            Ok(response) => response,
            Err(e) => {
                format!("Sorry, I couldn't generate a response. I encountered this Error {}", &e.to_string())
            }
        }
                
    }
    pub fn new(context: Context, message: Message) -> Self {
        Tutoro { context, message }
    }
    async fn send_response_to_message(&self, response: String ) -> Result<serenity::all::Message, serenity::Error>{
        self.message.channel_id.say(&self.context.http, response).await
    }
}