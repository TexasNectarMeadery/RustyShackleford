//--- IMPORTS ---//
use std::{env, fs::File, io::Read};

use serenity::{
    async_trait,
    framework::standard::macros::{command, group},
    framework::standard::{CommandResult, StandardFramework},
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

mod config;
//--- END IMPORTS ---//

//--- STRUCTS ---//
#[group]
#[commands(help, about)]
struct General;

struct Handler;
//--- END STRUCTS ---//

//--- COMMANDS ---//
const HELP_MESSAGE: &str = "Okay Boomer";

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, HELP_MESSAGE).await?;

    Ok(())
}

#[command]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    let filename = "README.md";
    let mut content = String::new();
    const TRY_HELP: &str = "\n Try typing '+help'";
    // Open the file in read-only mode.
    match File::open(filename) {
        Ok(mut file) => {
            // Read all the file content into a variable (ignoring the result of the operation).
            file.read_to_string(&mut content).unwrap();
            // The file is automatically closed when is goes out of scope.
        }

        Err(error) => {
            println!("Error opening file {}: {}", filename, error);
        }
    }
    if let Err(error) = msg
        .channel_id
        .say(&ctx.http, format!("{}{}", content, TRY_HELP))
        .await
    {
        println!("Error sending message: {:?}", error);
    }

    Ok(())
}
//--- END COMMANDS --//

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
