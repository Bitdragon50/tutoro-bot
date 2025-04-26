use std::env;
use dotenv::dotenv;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use crate::tutoro::Tutoro;
mod ollama;
mod tutoro;
mod vertex;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();
    println!("{}",env::var("CREDENTIAL_PATH").expect("Expected CREDENTIAL_PATH in the environment"));
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::DIRECT_MESSAGES;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    match client.start().await {
        Ok(()) => println!("Client started successfully"),
        Err(why) => println!("Client error: {:?}", why)
    }
}


struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        println!("user:{}\nmessage: {}", msg.author.name, msg.content);
        if !msg.mentions.is_empty() && msg.mentions_me(&ctx.http()).await.unwrap() {
            println!("mentions: {:?}", &msg.mentions[0].name);
            let tutor = Tutoro::new("ollama", ctx.clone(), msg.clone());
            tutor.respond_to_message().await;            
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}