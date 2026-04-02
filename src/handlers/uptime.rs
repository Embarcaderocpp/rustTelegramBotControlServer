use sysinfo::System;
use teloxide::{Bot, prelude::*, RequestError};

pub async fn uptime(bot: Bot, msg: Message) -> Result<(), RequestError> {
    let info  = system_uptime_info();
    bot.send_message(msg.chat.id, info).parse_mode(teloxide::types::ParseMode::Html)
        .await?;
    Ok(())
}

fn system_uptime_info() -> String {
    let uptime_sec = System::uptime();

    let days = uptime_sec / 86_400;
    let hours = (uptime_sec % 86_400) / 3_600;
    let minutes = (uptime_sec % 3_600) / 60;
    let seconds = uptime_sec % 60;

    format!(
        "⏱️  <b>Время работы системы</b>\n\
         
         Система работает уже:\n\
         <b>{} дн. {} ч. {} мин. {} сек.</b>\n\n\
         (всего {} секунд)",
        days, hours, minutes, seconds, uptime_sec
    )
}