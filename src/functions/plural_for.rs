pub fn plural_for(w: &str, l: usize) -> String {
    if l == 1 {
        w.to_string()
    } else {
        format!(
            "{}s",
            w
        )
    }
}