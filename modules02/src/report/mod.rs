pub fn summary() -> String {
    line(crate::labels::TOTAL, crate::arithmetic::sample_total())
}

fn line(label: &str, value: i32) -> String {
    format!("{label}: {value}")
}
