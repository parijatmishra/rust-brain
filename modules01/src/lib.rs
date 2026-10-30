mod labels {
    pub(crate) const TOTAL: &str = "TOTAL";
}
pub mod arithmetic {
    pub mod addition {
        pub fn add(a: i32, b: i32) -> i32 {
            super::combine(a, b)
        }
    }
    pub fn double(a: i32) -> i32 {
        addition::add(a, a)
        // self::addition::add(a, b) // alternative, optional, matter of style
    }
    fn combine(a: i32, b: i32) -> i32 {
        a + b
    }
    pub(crate) fn sample_total() -> i32 {
        addition::add(2, 3)
    }
}
pub mod report {
    pub fn summary() -> String {
        line(crate::labels::TOTAL, crate::arithmetic::sample_total())
    }
    fn line(label: &str, value: i32) -> String {
        format!("{label}: {value}")
    }
}
pub use report::summary;
