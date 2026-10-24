use cargo01::my_add;
use std::time::Instant;

fn main() {
    const ITERATIONS: u32 = 1_000_000;

    let start = Instant::now();
    for i in 0..ITERATIONS {
        std::hint::black_box(my_add(i as i32, 1));
    }
    println!("add:  {} iterations in {:?}", ITERATIONS, start.elapsed());
}
