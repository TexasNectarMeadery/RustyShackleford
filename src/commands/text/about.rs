//--- IMPORTS ---//
use std::{fs::File, io::Read};

use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::Context,
};
//--- END IMPORTS ---//

const TRY_HELP: &str = "\n Try typing '+help'";

#[command]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(error) = msg
        .channel_id
        .say(&ctx.http, format!("{}{}", parse_readme(), TRY_HELP))
        .await
    {
        println!("Error sending message: {:?}", error);
    }

    Ok(())
}

// Slash command
pub fn about_resp() -> String {
    return parse_readme();
}

// Parse README.md
fn parse_readme() -> String {
    let filename = "README.md";
    let mut content = String::new();
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
    return content;
}