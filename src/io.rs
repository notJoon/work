use std::{fs, io, path::PathBuf};

pub fn read_file(path: &PathBuf) -> io::Result<String> {
    fs::read_to_string(path)
}

pub fn write_file(path: &PathBuf, content: &str) -> io::Result<()> {
    fs::write(path, content)
}
