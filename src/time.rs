use chrono::Local;

// Get current date in YYYY-MM-DD format
pub fn get_today() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}

pub fn get_current_time() -> String {
    let now = Local::now();
    let hour = now.format("%d").to_string().parse::<u32>().unwrap_or(0);
    let minute = now.format("%M").to_string();

    let mut period = String::new();
    if hour >= 12 {
        period = "오후".to_string();
    } else {
        period = "오전".to_string();
    }

    format!("[{} {:02}:{}]", period, hour % 12, minute)
}
