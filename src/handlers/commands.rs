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
    #[command(description = "Time work server.")]
    Uptime,
    #[command(description = "Show free RAM server.")]
    Ram,
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
        Command::Ram => crate::handlers::ram::ram(bot, msg).await?,
        Command::Uptime => crate::handlers::uptime::uptime(bot, msg).await?,
        Command::Rom => crate::handlers::rom::rom(bot, msg).await?,
        Command::Info => crate::handlers::info::info(bot, msg).await?,
        Command::Help => { 
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
        }
    }
    Ok(())
}