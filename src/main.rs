//--- IMPORTS ---//
use std::env;

use serenity::{
    async_trait, framework::standard::macros::group, framework::standard::StandardFramework,
    model::gateway::Ready, prelude::*,
};

pub mod commands;
pub mod config;

pub use commands::text::{about, help};

//Static imports for commands
use crate::commands::ABOUT_COMMAND;
use crate::commands::HELP_COMMAND;
//--- END IMPORTS ---//

//--- STRUCTS ---//
#[group]
#[commands(help, about)]
struct General;

struct Handler;
//--- END STRUCTS ---//

//--- BOT ---//
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // configure auth token
    config::config::main();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // configure framework
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("+")) // set the bot's prefix to "+"
        .group(&GENERAL_GROUP);

    // login
    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_PRESENCES
        | GatewayIntents::GUILD_MEMBERS;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");

    // connection error handler
    if let Err(error) = client.start().await {
        println!("Client error: {:?}", error);
    }
}
//--- END BOT ---//
