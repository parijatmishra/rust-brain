use cargo01::word_count;

fn main() {
    let s = "Rust refresher: fast and concise.";
    println!("word_count({s:?}) = {}", word_count(s));
}
