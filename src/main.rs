mod cli;
mod editor;
mod io;
mod md;
mod time;

use clap::Parser;

use cli::{Cli, Command};
use editor::{
    edit_content, ensure_today_section, extract_section, find_section, insert_section,
    replace_section,
};
use io::{get_tag_file, read_file, write_file};
use md::format_bullets;
use time::get_current_time;

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli.command) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(command: Command) -> std::io::Result<()> {
    match command {
        Command::Todo => cmd_todo(),
        Command::Done => cmd_done(),
        Command::Note { tag } => cmd_note(tag),
    }
}

/// Edit TODO section for today
fn cmd_todo() -> std::io::Result<()> {
    let path = get_tag_file();
    let content = read_file(&path).unwrap_or_default();
    let content = ensure_today_section(&content);

    // Extract or create TODO section
    let todo_content = extract_section(&content, "TODO").unwrap_or_default();

    // Edit in editor
    let edited = edit_content(&todo_content)?;
    let formatted = format_bullets(&edited);

    // Update file
    let new_content = if find_section(&content, "TODO").is_some() {
        replace_section(&content, "TODO", &formatted)
    } else {
        insert_section(&content, "TODO", &formatted)
    };

    write_file(&path, &new_content)?;
    Ok(())
}

/// Add completed item with timestamp
fn cmd_done() -> std::io::Result<()> {
    let path = get_tag_file();
    let content = read_file(&path).unwrap_or_default();
    let content = ensure_today_section(&content);

    // Edit in editor
    let edited = edit_content("")?;
    if edited.trim().is_empty() {
        return Ok(());
    }

    let formatted = format_bullets(&edited);
    let timestamp = get_current_time();

    // Insert timestamped section
    let new_content = insert_section(&content, &timestamp, &formatted);

    write_file(&path, &new_content)?;
    Ok(())
}

/// Add a note with optional tag
fn cmd_note(tag: Option<String>) -> std::io::Result<()> {
    let path = get_tag_file();
    let content = read_file(&path).unwrap_or_default();
    let content = ensure_today_section(&content);

    // Edit in editor
    let edited = edit_content("")?;
    if edited.trim().is_empty() {
        return Ok(());
    }

    let formatted = format_bullets(&edited);

    // Determine header
    let header = match tag {
        Some(t) => format!("#{}", t),
        None => "노트:".to_string(),
    };

    // Insert note section
    let new_content = insert_section(&content, &header, &formatted);

    write_file(&path, &new_content)?;
    Ok(())
}
