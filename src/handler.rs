use serenity::{
    async_trait, 
    client::{ Context, EventHandler },
    model::{ gateway::Ready, channel::Message }, 
    utils::{MessageBuilder},
};
use crate::commands;

pub struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        //check for bot and to see if we need to respond at all.
        let mut msg_content = msg.content;
        if msg.author.bot { return; }
        if !msg_content.starts_with(".") { return; }
        else { msg_content.remove(0); }
        let commands = commands::Commands::new(msg_content);
        let response;
        match commands.is_valid {
            true => {
                response = MessageBuilder::new()
                .push(commands.response)
                .build();
            },
            _ => return,
        }


        if let Err(why) = msg.channel_id.say(&context.http, &response).await {
                println!("Error sending message: {:?}", why);
        }
        
 
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
    }
}
    