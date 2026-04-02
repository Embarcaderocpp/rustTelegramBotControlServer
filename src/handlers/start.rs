use teloxide::{Bot, prelude::*, RequestError};

pub async fn start(bot: Bot, msg: Message) -> Result<(), RequestError> {
    bot.send_message(msg.chat.id, "U press Start").await?;
    Ok(())
}