use serenity::{async_trait, client::{ Context, EventHandler }, model::{ gateway::Ready, channel::Message }, prelude::*, utils::{Content, MessageBuilder}};

pub struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        let mut res_content;
        match msg.content.as_str() {
            "*help" => res_content = "you said help", 
            _ => return,
        };
        let response = MessageBuilder::new()
            .push(res_content)
            .build();

        if let Err(why) = msg.channel_id.say(&context.http, &response).await {
                println!("Error sending message: {:?}", why);
        }
 
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
    }
}