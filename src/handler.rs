use serenity::{
    async_trait, 
    client::{ Context, EventHandler },
    model::{ gateway::Ready, channel::Message }, 
    utils::{MessageBuilder},
};
use crate::commands;
use crate::vars;
pub struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        //check for bot and to see if we need to respond at all.
        let guild_id = msg.guild_id.expect("Was not able to get guildID").to_string();
        let vars: vars::Vars = vars::Vars::get_server_vars(&guild_id);
        let mut msg_content = msg.content;
        if msg.author.bot || !msg_content.starts_with(&vars.start_command) { return; }
        else { msg_content.remove(0); }
        let commands = commands::Commands::new(&msg_content, &guild_id);
        let response;
        if commands.is_valid { response = MessageBuilder::new().push(commands.response).build(); }
        else { return }
        if let Err(why) = msg.channel_id.say(&context.http, &response).await {
                println!("Error sending message: {:?}", why);
        } 
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
    }
}
    