use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("input x");
    let mut x = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("ERROR");

    println!("input y");
    let mut y = String::new();
    io::stdin()
        .read_line(&mut y)
        .expect("ERROR");

    let z = rand::thread_rng().gen_range(1..=10);
    println!("guess what is z (1-10)");
    let mut z1 = String::new();
    io::stdin()
        .read_line(&mut z1)
        .expect("ERROR");

    println!("x={}\ny={}", x, y);

    match z1.cmp(&z) {
        Ordering::less => println!("Your answer is too small!\nThe Right Answer is {z}");
        Ordering::Greater => println!("Your answer is too big!\nThe Right Answer is {z}");
        Ordering::Equal => println!("You guessed the right z!");
    }
}
