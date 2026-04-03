use teloxide::{Bot, prelude::*, RequestError};
use teloxide::utils::command::BotCommands;
use crate::handlers::commands::Command;

pub async fn start(bot: Bot, msg: Message) -> Result<(), RequestError> {
    bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
    Ok(())
}