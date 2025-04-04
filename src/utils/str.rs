pub fn pad_left(s: &str, length: usize, pad_char: char) -> String {
    if s.len() >= length {
        return s.to_string();
    }
    let padding = pad_char.to_string().repeat(length - s.len());
    format!("{}{}", padding, s)
}
