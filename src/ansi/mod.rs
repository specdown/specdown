pub fn strip_ansi_escape_chars(string: &str) -> String {
    String::from_utf8_lossy(&strip_ansi_escapes::strip(string)).to_string()
}

#[cfg(test)]
mod tests {
    use super::strip_ansi_escape_chars;

    #[test]
    fn strips_ansi_escape_chars() {
        assert_eq!(
            "green text",
            strip_ansi_escape_chars("\x00\x1b[32mgreen text")
        );
    }

    #[test]
    fn preserves_unicode_chars() {
        assert_eq!("\u{2713}", strip_ansi_escape_chars("\u{2713}"));
    }
}
