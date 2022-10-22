use std::cmp::Ordering;
use std::io;

fn main() {
    println!("a1");

    function(23145);
}

fn function(x: u32) {
    let y = 56;
    println!("{x}");
    let z = x + y;
    println!("{z}")
}