use sysinfo::{Disks, System};
use teloxide::{Bot, RequestError};
use teloxide::prelude::*;
use teloxide::types::Message;

pub async fn info(bot: Bot, msg: Message) -> Result<(), RequestError> {
    let info = get_system_info();
    bot.send_message(msg.chat.id, info)
        .parse_mode(teloxide::types::ParseMode::Html)
        .await?;
    Ok(())
}

fn get_system_info() -> String {
    let mut sys = System::new_all();

    // Важно: делаем два обновления CPU, чтобы получить корректный процент использования
    sys.refresh_all();
    std::thread::sleep(std::time::Duration::from_millis(300)); // небольшая пауза для точности
    sys.refresh_cpu_usage();   // обновляем только CPU usage

    let mut text = String::from("<b>📊 Состояние сервера</b>\n\n");

    // ==================== CPU ====================
    text.push_str("<b>🧠 Процессор</b>\n");
    let total_cpu = sys.global_cpu_usage();
    text.push_str(&format!("Общее использование: <b>{:.1}%</b>\n\n", total_cpu));

    text.push_str("По ядрам:\n");
    for (i, cpu) in sys.cpus().iter().enumerate() {
        text.push_str(&format!("  Ядро {}: <b>{:.1}%</b>\n", i, cpu.cpu_usage()));
    }
    text.push_str("\n");

    // ==================== RAM ====================
    let total_ram = sys.total_memory();
    let used_ram = sys.used_memory();
    let available_ram = sys.available_memory();

    let total_ram_gb = bytes_to_gb(total_ram);
    let used_ram_gb = bytes_to_gb(used_ram);
    let avail_ram_gb = bytes_to_gb(available_ram);
    let ram_percent = if total_ram > 0 {
        (used_ram as f64 / total_ram as f64) * 100.0
    } else {
        0.0
    };

    text.push_str("<b>💾 Оперативная память (RAM)</b>\n");
    text.push_str(&format!(
        "Всего:     <b>{:.2}</b> ГБ\n\
         Занято:    <b>{:.2}</b> ГБ\n\
         Свободно:  <b>{:.2}</b> ГБ\n\
         Использовано: <b>{:.1}%</b>\n\n",
        total_ram_gb, used_ram_gb, avail_ram_gb, ram_percent
    ));

    // ==================== ROM (Диск) ====================
    let disks = Disks::new_with_refreshed_list();
    if let Some(disk) = disks.list().first() {
        let total = disk.total_space();
        let available = disk.available_space();
        let used = total.saturating_sub(available);

        let total_gb = bytes_to_gb(total);
        let avail_gb = bytes_to_gb(available);
        let used_gb = bytes_to_gb(used);
        let disk_percent = if total > 0 {
            (used as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        let name = disk.name().to_string_lossy();
        let mount = disk.mount_point().to_string_lossy();

        text.push_str("<b>🗄️ Диск (ROM)</b>\n");
        text.push_str(&format!(
            "Диск: <b>{}</b>\n\
             Монтирование: <b>{}</b>\n\
             Всего:     <b>{:.2}</b> ГБ\n\
             Свободно:  <b>{:.2}</b> ГБ\n\
             Занято:    <b>{:.2}</b> ГБ\n\
             Использовано: <b>{:.1}%</b>",
            name, mount, total_gb, avail_gb, used_gb, disk_percent
        ));
    } else {
        text.push_str("❌ Информация о диске недоступна");
    }

    text
}

fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / (1024.0 * 1024.0 * 1024.0)
}