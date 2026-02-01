use chrono::Local;

// Get current date in YYYY-MM-DD format
pub fn get_today() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}

pub fn get_current_time() -> String {
    let now = Local::now();
    let hour = now.format("%H").to_string().parse::<u32>().unwrap_or(0);
    let minute = now.format("%M").to_string();

    let period = if hour >= 12 { "오후" } else { "오전" };

    format!("[{} {:02}:{}]", period, hour, minute)
}
