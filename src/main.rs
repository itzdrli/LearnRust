use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {


    loop {
        // 用户指定x值
        println!("input x");
        let mut x = String::new();
        io::stdin()
            .read_line(&mut x)
            .expect("ERROR");
        let x: u32 = match x.trim().parse() {
            Ok(num) => {
                println!("x = {x}");
                break;
            }
            Err(_) => continue,
        };
    }

    loop {
        // 用户指定y值
        println!("input y");
        let mut y = String::new();
        io::stdin()
            .read_line(&mut y)
            .expect("ERROR");
        let y: u32 = match y.trim().parse() {
            Ok(num) => {
                println!("y = {y}");
                break;
            }
            Err(_) => continue,
        };
    }

    // 随机生成z值
    let z = rand::thread_rng().gen_range(1..=10);

    loop {
            // 用户输入猜测的z值
            println!("input the z you guessed");
            let mut z1 = String::new();
            io::stdin()
                .read_line(&mut z1)
                .expect("ERROR");

            // 用户的z值转型为数字，如果不是数字就报错
            let z1: u32 = match z1.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            // 将正确z值与用户输入的z值进行比较并输出结果
            match z1.cmp(&z) {
                Ordering::Less => println!("Your answer is too small!\nThe Right Answer is {z}"),
                Ordering::Greater => println!("Your answer is too big!\nThe Right Answer is {z}"),
                Ordering::Equal => {
                    println!("You guessed the right z!");
                    break;
                },
            }
    }
}
