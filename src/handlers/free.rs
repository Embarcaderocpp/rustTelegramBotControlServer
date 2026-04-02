use sysinfo::{MemoryRefreshKind, System};
use teloxide::{Bot, prelude::*, RequestError};

pub async fn free(bot: Bot, msg: Message) -> Result<(), RequestError> {
    let info = memory_info();
    bot.send_message(msg.chat.id, info).await?;
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
        "  Память системы\n\
          Общая:     {:.2} GB \n\
          Занято:    {:.2} GB  (  {:.1}% )\n\
          Доступно:  {:.2} GB \n\
          Свободно:  {:.2} GB  ",
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