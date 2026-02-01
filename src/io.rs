use std::{env, fs, io, path::PathBuf};

pub fn read_file(path: &PathBuf) -> io::Result<String> {
    fs::read_to_string(path)
}

pub fn write_file(path: &PathBuf, content: &str) -> io::Result<()> {
    fs::write(path, content)
}

pub fn get_tag_file() -> PathBuf {
    env::var("TAG_FILE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("todo.txt")
        })
}

pub fn get_editor() -> String {
    env::var("EDITOR")
        .or_else(|_| env::var("VISUAL"))
        .unwrap_or_else(|_| "vim".to_string())
}