use crate::time::get_today;

fn has_today_section(content: &str) -> bool {
    content.lines().any(|line| line == get_today())
}

fn get_today_section_start(content: &str) -> Option<usize> {
    content.lines().position(|line| line == get_today())
}

/// line index of the end of today's section (0 based, exclusive)
fn get_today_section_end(content: &str, start_line: usize) -> usize {
    let lines: Vec<&str> = content.lines().collect();
    let date_pattern = |s: &str| {
        s.len() == 10
            && s.chars().nth(4) == Some('-')
            && s.chars().nth(7) == Some('-')
            && s.chars().take(4).all(|c| c.is_ascii_digit())
    };

    for i in (start_line + 1)..lines.len() {
        if date_pattern(lines[i]) {
            return i;
        }
    }

    lines.len()
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
