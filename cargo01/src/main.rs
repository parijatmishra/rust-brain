// ANCHOR: main
use cargo01::my_add;
use cargo01_greet::greet;

fn main() {
    println!("my_add(2, 2) = {}", my_add(2, 2));
    greet("Rust refresher");
}
// ANCHOR_END: main
