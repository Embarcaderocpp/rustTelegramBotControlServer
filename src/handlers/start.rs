use teloxide::{Bot, prelude::*, RequestError};

pub async fn start(bot: Bot, msg: Message) -> Result<(), RequestError> {
    bot.send_message(msg.chat.id, "You press Start!").await?;
    Ok(())
}