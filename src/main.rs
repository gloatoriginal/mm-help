use serenity::{
    client::Client, 
};

use std::env;
mod commands;
mod handler;
pub mod vars;
pub mod sqlpool;

#[tokio::main]
async fn main() {

    // Login with a bot token from the environment
	let token = env::var("TOKE").expect("Forgot to add token to env vars");

    //create the client in order to handle requests
    //this is acting as a user logging into the bot with a token
    //.await is similar to JS await/async
    let mut client = Client::builder(&token)
        //enter into the impl handler.rs, that's where messaging will occur.
        .event_handler(handler::Handler)
        .await
        .expect("Error creating client");
    //the client starts, I need to maybe start looking into sharding,
    //Sharding would *i think* allow multiple threads to open up and be multi-threaded
    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

