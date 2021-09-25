use serenity::{
    client::Client, 
};
use std::env;
mod commands;
mod handler;


#[tokio::main]
async fn main() {
    println!("change from remote repo");
    /*let framework = StandardFramework::new()
        .configure(|c| c.prefix("*")); // set bot prefix to *
*/
    // Login with a bot token from the environment
	let token = env::var("TOKE").expect("Forgot to add token to env vars");
    let mut client = Client::builder(&token)
        //enter into the impl handler.rs, that's where messaging will occur.
        .event_handler(handler::Handler)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

