pub fn truncate(text: &String, size: usize) -> String {
    if text.len() > size {
        return format!("{}{}", &text[0..size - 1], "…");
    }

    text.into()
}
