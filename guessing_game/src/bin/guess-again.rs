use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("enter guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("input failed");
        let guess: u32 = match guess.trim().parse() {
            Err(_) => continue,
            Ok(n) => n,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too low"),
            Ordering::Greater => println!("too high"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
