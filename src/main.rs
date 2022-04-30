//--- IMPORTS ---//
//Standard imports
use std::env;

// External imports
use serenity::{
    async_trait, framework::standard::macros::group, framework::standard::StandardFramework,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

// Internal imports
pub mod commands;
pub mod config;
pub mod utils;

pub use commands::text::{about, help};
pub use utils::{regex_lib, math_lib};

// Static imports for commands
use crate::commands::ABOUT_COMMAND;
use crate::commands::HELP_COMMAND;
//--- END IMPORTS ---//

//--- STRUCTS ---//
#[group]
#[commands(help, about)]
struct General;

struct Handler;
//--- END STRUCTS ---//

//--- CONSTANTS ---//
const BOT_ID: &str = "967821729818877962";
const CONER_ID: &str = "307630851884318720";
const CONER_NUM: i32 = 69;
//--- END CONSTANTS ---//

//--- BOT ---//

// Generic send message
async fn send_message(ctx: Context, msg: Message, text: &str) {
    if let Err(why) = msg.channel_id.say(&ctx.http, text).await {
        println!("Error sending message: {:?}", why);
    }
}

#[async_trait]
impl EventHandler for Handler {
    // Message handler
    async fn message(&self, ctx: Context, msg: Message) {

        // Coner handler
        if msg.author.id.to_string() == CONER_ID {
            let rand_val = math_lib::get_rand_num();
            if rand_val == CONER_NUM {
                send_message(ctx.clone(), msg.clone(), "hi coner ;)").await;
            }
        }

        // Regex handler
        if msg.author.id.to_string() != BOT_ID {
            let mut regex_dict = regex_lib::LibRegex::new();
            regex_dict.build_regex_map();
            if regex_dict.regex_map.len() > 0 {
                let resp = regex_dict.regex_search(msg.content.as_str());
                if resp != "" {
                    send_message(ctx, msg, resp).await;
                }
            }
        }
    }
    
    // Connect to discord server
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
