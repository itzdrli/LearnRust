// use std::io;

fn main() {
    let mut x = 0;

    let y = loop {
        x += 1;
        if x == 10 {
            break x;
        }
    };
    println!("{y}");
}

