//--- IMPORTS ---//
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::Context,
};
//--- END IMPORTS ---//

const HELP_MESSAGE: &str = "Okay Boomer";

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, HELP_MESSAGE).await?;

    Ok(())
}

// Slash command
pub fn help_resp() -> String {
    return HELP_MESSAGE.to_string();
}
