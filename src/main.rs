use std::{env, path::PathBuf};

use chrono::Local;

mod io;

fn get_todo_file() -> PathBuf {
    env::var("TODO_FILE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("todo.txt")
        })
}

fn get_editor() -> String {
    env::var("EDITOR")
        .or_else(|_| env::var("VISUAL"))
        .unwrap_or_else(|_| "vim".to_string())
}

// Get current date in YYYY-MM-DD format
fn get_today() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}

fn get_current_time() -> String {
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

fn get_bullet(depth: usize) -> String {
    if depth % 2 == 0 {
        "-".to_string()
    } else {
        ".".to_string()
    }
}

fn main() {
    println!("Hello, world!");
}
