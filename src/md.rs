pub fn get_bullet(depth: usize) -> String {
    if depth % 2 == 0 {
        "-".to_string()
    } else {
        ".".to_string()
    }
}
