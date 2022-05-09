//--- IMPORTS ---//
//Standard imports
use std::env;

// External imports
use serenity::{
    async_trait, framework::standard::macros::group, framework::standard::StandardFramework,
    model::{channel::Message, gateway::Ready, id::GuildId},
    prelude::*,
    model::interactions::application_command::{
        ApplicationCommand,
        ApplicationCommandInteractionDataOptionValue,
        ApplicationCommandOptionType,
    },
    model::interactions::{Interaction, InteractionResponseType}
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

    // Slash Commands
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => "Hey, I'm alive!".to_string(),
                "help" => help::help_resp(),
                "about" => about::about_resp(),
                "id" => {
                    let options = command
                        .data
                        .options
                        .get(0)
                        .expect("Expected user option")
                        .resolved
                        .as_ref()
                        .expect("Expected user object");

                    if let ApplicationCommandInteractionDataOptionValue::User(user, _member) =
                        options
                    {
                        format!("{}'s id is {}", user.tag(), user.id)
                    } else {
                        "Please provide a valid user".to_string()
                    }
                },
                "attachmentinput" => {
                    let options = command
                        .data
                        .options
                        .get(0)
                        .expect("Expected attachment option")
                        .resolved
                        .as_ref()
                        .expect("Expected attachment object");

                    if let ApplicationCommandInteractionDataOptionValue::Attachment(attachment) =
                        options
                    {
                        format!(
                            "Attachment name: {}, attachment size: {}kb",
                            attachment.filename, attachment.size
                        )
                    } else {
                        "Please provide a valid attachment".to_string()
                    }
                },
                "welcome" => {
                    let user_option = command
                        .data
                        .options
                        .get(0)
                        .expect("Expected user option")
                        .resolved
                        .as_ref()
                        .expect("Expected user object");
                    let message_option = command
                        .data
                        .options
                        .get(1)
                        .expect("Expected string option")
                        .resolved
                        .as_ref()
                        .expect("Expected string object");

                    if let ApplicationCommandInteractionDataOptionValue::User(user, _member) =
                        user_option
                    {
                        if let ApplicationCommandInteractionDataOptionValue::String(message, ) =
                        message_option
                        {
                            format!("Hey {}! {}", user.name, message)
                        } else {
                            "Please provide a valid string".to_string()
                        }
                    } else {
                        "Please provide a valid user".to_string()
                    }
                },
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }
    
    // Connect to discord server
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        config::config::guild();
        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let _commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("ping").description("A ping command")
                })
                .create_application_command(|command| {
                    command.name("help").description("Get helpful information on Rusty")
                })
                .create_application_command(|command| {
                    command.name("about").description("Learn about Rusty")
                })
                .create_application_command(|command| {
                    command.name("id").description("Get a user id").create_option(|option| {
                        option
                            .name("id")
                            .description("The user to lookup")
                            .kind(ApplicationCommandOptionType::User)
                            .required(true)
                    })
                })
                .create_application_command(|command| {
                    command
                        .name("welcome")
                        .description("Welcome a user")
                        .create_option(|option| {
                            option
                                .name("user")
                                .description("The user to welcome")
                                .kind(ApplicationCommandOptionType::User)
                                .required(true)
                        })
                        .create_option(|option| {
                            option
                                .name("message")
                                .description("The message to send")
                                .kind(ApplicationCommandOptionType::String)
                                .required(true)
                                .add_string_choice(
                                    "Welcome to the club",
                                    "hey buddy",
                                )
                                .add_string_choice("Hey, do you want a coffee?", "coffee",)
                        })
                })
                .create_application_command(|command| {
                    command
                        .name("numberinput")
                        .description("Test command for number input")
                        .create_option(|option| {
                            option
                                .name("int")
                                .description("An integer from 5 to 10")
                                .kind(ApplicationCommandOptionType::Integer)
                                .min_int_value(5)
                                .max_int_value(10)
                                .required(true)
                        })
                        .create_option(|option| {
                            option
                                .name("number")
                                .description("A float from -3.3 to 234.5")
                                .kind(ApplicationCommandOptionType::Number)
                                .min_number_value(-3.3)
                                .max_number_value(234.5)
                                .required(true)
                        })
                })
                .create_application_command(|command| {
                    command
                        .name("attachmentinput")
                        .description("Test command for attachment input")
                        .create_option(|option| {
                            option
                                .name("attachment")
                                .description("A file")
                                .kind(ApplicationCommandOptionType::Attachment)
                                .required(true)
                        })
                })
        })
        .await;

        //Uncomment line below to debug guild slash command creation
        //println!("I now have the following guild slash commands: {:#?}", commands);

        let _guild_commandss =
            ApplicationCommand::create_global_application_command(&ctx.http, |command| {
                command.name("wonderful_command").description("An amazing command")
            })
            .await;

        //Uncomment line below to debug global slash command creation
        //println!("I created the following global slash command: {:#?}", guild_commands);
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
