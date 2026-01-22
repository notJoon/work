use std::{env, fs, io, path::PathBuf};
use std::io::Read;

pub fn read_file(path: &PathBuf) -> io::Result<String> {
    fs::read_to_string(path)
}

pub fn write_file(path: &PathBuf, content: &str) -> io::Result<()> {
    fs::write(path, content)
}

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

fn read_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}