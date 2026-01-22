/// Format bullet points
/// indentation based, alternating between '-' and '.'
pub fn format_bullets(content: &str) -> String {
    let mut result = Vec::new();

    for line in content.lines() {
        if line.is_empty() {
            result.push(String::new());
            continue;
        }

        let stripped = line.trim_start();
        let indent_count = line.len() - stripped.len();

        // check if line starts with desired bullet points
        if stripped.starts_with("- ") || stripped.starts_with(". ") || stripped.starts_with("* ") {
            let text = &stripped[2..];
            let bullet = get_bullet(indent_count);
            let indent = " ".repeat(indent_count);
            result.push(format!("{}{} {}", indent, bullet, text));
        } else {
            result.push(line.to_string());
        }
    }

    result.join("\n")
}

pub fn get_bullet(depth: usize) -> String {
    if depth % 2 == 0 {
        "-".to_string()
    } else {
        ".".to_string()
    }
}
