use std::io::{self, Write};
use std::process::Command;

use tempfile::NamedTempFile;

use crate::io::get_editor;
use crate::time::get_today;

pub fn has_today_section(content: &str) -> bool {
    content.lines().any(|line| line == get_today())
}

fn get_today_section_start(content: &str) -> Option<usize> {
    content.lines().position(|line| line == get_today())
}

/// line index of the end of today's section (0 based, exclusive)
fn get_today_section_end(content: &str, start_line: usize) -> usize {
    let lines: Vec<&str> = content.lines().collect();

    for i in (start_line + 1)..lines.len() {
        if is_date_line(lines[i]) {
            return i;
        }
    }

    lines.len()
}

fn is_date_line(s: &str) -> bool {
    s.len() == 10
        && s.chars().nth(4) == Some('-')
        && s.chars().nth(7) == Some('-')
        && s.chars().take(4).all(|c| c.is_ascii_digit())
}

/// Create new section for today
pub fn create_today_section(content: &str) -> String {
    let today = get_today();
    let mut result = String::new();

    result.push_str("\n\n\n");
    result.push_str(&today);
    result.push('\n');
    result.push_str("==========\n");
    result.push('\n');

    // add existing content (removing leading newlines)
    let trimmed = content.trim_start_matches('\n');
    result.push_str(trimmed);

    result
}

/// Open editor with initial content and return edited content
pub fn edit_content(initial: &str) -> io::Result<String> {
    let mut temp_file = NamedTempFile::new()?;
    temp_file.write_all(initial.as_bytes())?;
    temp_file.flush()?;

    let editor = get_editor();
    let path = temp_file.path();

    let status = Command::new(&editor).arg(path).status()?;

    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Editor '{}' exited with error", editor),
        ));
    }

    std::fs::read_to_string(path)
}

/// Find a section by header pattern in today's section
/// Returns (start_line, end_line) where start_line is the header line
pub fn find_section(content: &str, header: &str) -> Option<(usize, usize)> {
    let today_start = get_today_section_start(content)?;
    let today_end = get_today_section_end(content, today_start);
    let lines: Vec<&str> = content.lines().collect();

    // Search for header within today's section (skip date and underline)
    for i in (today_start + 2)..today_end {
        if lines[i] == header {
            // Find end of this section (next header or end of today)
            let section_end = find_next_section_header(&lines, i + 1, today_end);
            return Some((i, section_end));
        }
    }

    None
}

/// Find next section header (TODO, timestamp, or tag)
fn find_next_section_header(lines: &[&str], start: usize, end: usize) -> usize {
    for i in start..end {
        let line = lines[i].trim();
        if line == "TODO"
            || line.starts_with('[')
            || line.starts_with('#')
            || line.starts_with("노트:")
        {
            return i;
        }
    }
    end
}

/// Extract content from a section (lines between header and next section)
pub fn extract_section(content: &str, header: &str) -> Option<String> {
    let (start, end) = find_section(content, header)?;
    let lines: Vec<&str> = content.lines().collect();

    // Get lines after header until end
    let section_lines: Vec<&str> = lines[(start + 1)..end].to_vec();
    Some(section_lines.join("\n"))
}

/// Replace a section's content (keeps header, replaces body)
pub fn replace_section(content: &str, header: &str, new_body: &str) -> String {
    if let Some((start, end)) = find_section(content, header) {
        let lines: Vec<&str> = content.lines().collect();
        let mut result = Vec::new();

        // Lines before section
        result.extend(lines[..=start].iter().map(|s| s.to_string()));

        // New body
        for line in new_body.lines() {
            result.push(line.to_string());
        }

        // Lines after section
        result.extend(lines[end..].iter().map(|s| s.to_string()));

        result.join("\n")
    } else {
        content.to_string()
    }
}

/// Insert a new section at the end of today's section
pub fn insert_section(content: &str, header: &str, body: &str) -> String {
    let today_start = match get_today_section_start(content) {
        Some(s) => s,
        None => return content.to_string(),
    };
    let today_end = get_today_section_end(content, today_start);
    let lines: Vec<&str> = content.lines().collect();

    let mut result = Vec::new();

    // Lines before insertion point
    result.extend(lines[..today_end].iter().map(|s| s.to_string()));

    // Add new section
    result.push(String::new());
    result.push(header.to_string());
    for line in body.lines() {
        result.push(line.to_string());
    }

    // Lines after today's section
    if today_end < lines.len() {
        result.extend(lines[today_end..].iter().map(|s| s.to_string()));
    }

    result.join("\n")
}

/// Ensure today's section exists in content
pub fn ensure_today_section(content: &str) -> String {
    if has_today_section(content) {
        content.to_string()
    } else {
        create_today_section(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_content(date: &str) -> String {
        format!(
            "{date}\n==========\n\nTODO\n - Task 1\n - Task 2\n\n[오전 10:30]\n - Done item\n"
        )
    }

    #[test]
    fn test_is_date_line() {
        assert!(is_date_line("2026-02-01"));
        assert!(is_date_line("2025-12-31"));
        assert!(!is_date_line("not a date"));
        assert!(!is_date_line("2026-2-01")); // wrong format
        assert!(!is_date_line("26-02-01")); // wrong format
    }

    #[test]
    fn test_find_section() {
        let today = get_today();
        let content = make_test_content(&today);

        let result = find_section(&content, "TODO");
        assert!(result.is_some());

        let (start, end) = result.unwrap();
        assert_eq!(start, 3); // TODO is at line 3
        assert_eq!(end, 7); // Next section starts at line 7
    }

    #[test]
    fn test_extract_section() {
        let today = get_today();
        let content = make_test_content(&today);

        let result = extract_section(&content, "TODO");
        assert!(result.is_some());

        let extracted = result.unwrap();
        assert!(extracted.contains("Task 1"));
        assert!(extracted.contains("Task 2"));
    }

    #[test]
    fn test_replace_section() {
        let today = get_today();
        let content = make_test_content(&today);

        let new_content = replace_section(&content, "TODO", " - New task");
        assert!(new_content.contains("New task"));
        assert!(!new_content.contains("Task 1"));
    }

    #[test]
    fn test_insert_section() {
        let today = get_today();
        let content = make_test_content(&today);

        let new_content = insert_section(&content, "#TIL", " - Learned something");
        assert!(new_content.contains("#TIL"));
        assert!(new_content.contains("Learned something"));
    }

    #[test]
    fn test_create_today_section() {
        let content = "old content";
        let result = create_today_section(content);

        assert!(result.contains(&get_today()));
        assert!(result.contains("=========="));
        assert!(result.contains("old content"));
    }

    #[test]
    fn test_ensure_today_section_exists() {
        let today = get_today();
        let content = make_test_content(&today);

        let result = ensure_today_section(&content);
        assert_eq!(result, content);
    }

    #[test]
    fn test_ensure_today_section_creates() {
        let content = "old content";
        let result = ensure_today_section(content);

        assert!(result.contains(&get_today()));
    }
}
