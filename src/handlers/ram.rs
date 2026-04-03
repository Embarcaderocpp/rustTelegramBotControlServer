use sysinfo::{MemoryRefreshKind, System};
use teloxide::{Bot, prelude::*, RequestError};

pub async fn ram(bot: Bot, msg: Message) -> Result<(), RequestError> {
    let info = memory_info();
    bot.send_message(msg.chat.id, info).parse_mode(teloxide::types::ParseMode::Html)
        .await?;
    Ok(())
}

fn memory_info() -> String {
    let mut sys = System::new();
    sys.refresh_memory_specifics(MemoryRefreshKind::everything());

    let total = sys.total_memory();
    let used = sys.used_memory();
    let available = sys.available_memory();
    let free = sys.free_memory();

    let used_percent = (used as f64 / total as f64) * 100.0;

    format!(
        " <b>Память системы</b>\n\
             Общая:     <b>{:.2}</b> GB \n\
             Занято:    <b>{:.2}</b> GB  (  {:.1}% )\n\
             Доступно:  <b>{:.2}</b> GB \n\
             Свободно:  <b>{:.2}</b> GB  ",
        bytes_to_gb(total),
        bytes_to_gb(used),
        used_percent,
        bytes_to_gb(available),
        bytes_to_gb(free)
    )
}

fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / 1_073_741_824.0
}