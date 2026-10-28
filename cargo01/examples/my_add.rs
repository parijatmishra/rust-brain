use cargo01::{my_add, word_count};

fn main() {
    println!("my_add(2, 2) = {}", my_add(2, 2));
    let s = "Rust refresher: fast and concise.";
    println!("word_count({s:?}) = {}", word_count(s));
}
