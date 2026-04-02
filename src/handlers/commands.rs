use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use teloxide::Bot;
use teloxide::types::Message;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "Show help about bot.")]
    Help,
    #[command(description = "To start the bot.")]
    Start,
    #[command(description = "Show server status.")]
    Status,
    #[command(description = "Time work server.")]
    Time,
    #[command(description = "Show free RAM server.")]
    Free,
    #[command(description = "Show ROM server.")]
    Rom,
    #[command(description = "Server info.")]
    Info,
}

pub async fn command_handler(
    bot: Bot,
    msg: Message,
    cmd: Command,
) -> ResponseResult<()> {
    match cmd {
        Command::Start => crate::handlers::start::start(bot, msg).await?,
        Command::Free => {
            bot.send_message(msg.chat.id, "bot: Free").await?;
        }
        Command::Status => {
            bot.send_message(msg.chat.id, "bot: Status").await?;
        }
        Command::Time => {
            bot.send_message(msg.chat.id, "bot: Time").await?;
        }
        Command::Rom => {
            bot.send_message(msg.chat.id, "bot: Rom").await?;
        }
        Command::Info => {
            bot.send_message(msg.chat.id, "bot: Info").await?;
        }
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
        }
    }
    Ok(())
}