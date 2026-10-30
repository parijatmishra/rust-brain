pub mod addition;

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
