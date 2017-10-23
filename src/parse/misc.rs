pub fn format_char(ch: &char) -> String {
    match ch {
        &'\n' => String::from("\\n"),
        ch => format!("{}", ch),
    }
}
