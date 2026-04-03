use sysinfo::Disks;
use teloxide::{Bot, RequestError};
use teloxide::prelude::*;
use teloxide::types::Message;

pub async fn rom(bot: Bot, msg: Message) -> Result<(), RequestError> {
    let info = rom_info();
    bot.send_message(msg.chat.id, info)
        .parse_mode(teloxide::types::ParseMode::Html)
        .await?;
    Ok(())
}

fn rom_info() -> String {
    let disks = Disks::new_with_refreshed_list();

    if let Some(disk) = disks.list().first() {
        let total = disk.total_space();
        let available = disk.available_space();
        let used = total.saturating_sub(available);

        let name = disk.name().to_string_lossy();
        let mount = disk.mount_point().to_string_lossy();

        let total_gb = bytes_to_gb(total);
        let avail_gb = bytes_to_gb(available);
        let used_gb = bytes_to_gb(used);
        let percent = (used as f64 / total as f64) * 100.0;

        format!(
            "<b>💾 Информация о диске</b>\n\n\
             Диск: <b>{}</b>\n\
             Монтирование: <b>{}</b>\n\n\
             Всего:     <b>{:.2}</b> ГБ\n\
             Свободно:  <b>{:.2}</b> ГБ\n\
             Занято:    <b>{:.2}</b> ГБ\n\
             Использовано: <b>{:.1}%</b>",
            name, mount, total_gb, avail_gb, used_gb, percent
        )
    } else {
        "❌ Диски не найдены!".to_string()
    }
}

fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / (1024.0 * 1024.0 * 1024.0)
}