use rand::random;
use regex::Regex;
use std::io;
use std::io::Write;
use text_io::read;

fn ternary() {
    // if/else ternary
    let x: u8 = random();
    let a = if x % 2 == 0 { "even" } else { "odd" };
    println!("{} is {}", x, a);
}

fn simple_loop() {
    // loop forever
    let mut i = 1;
    loop {
        println!("looping {}", i);
        i += 1;
        if i > 5 {
            break;
        }
        // could also loop forever and let the user send signal interrupt
    }
}

fn loop_user_input() {
    println!("looping forever!");
    loop {
        // get y/n answer on same line as prompt
        print!("keep looping? [y/n] ");
        // gotta flush stdout since it's waiting for something to write
        io::stdout().flush().unwrap();

        // read in user response until (and not including) newline
        let answer: String = read!("{}\n");

        // match lowercase user input and act on it
        match answer.to_lowercase().as_str() {
            "y" => {
                println!("looping again");
            }
            "n" => {
                println!("done looping");
                break;
            }
            _ => {
                println!("wtf does {} mean?", answer);
            }
        }
    }
}

fn temp_unit_convert() {
    let temp_re = Regex::new("^(\\d+)(F|C|f|c)$").unwrap();
    let converted_temp = loop {
        print!("enter temp in farentheit (nF) or celcius (nC) ");
        io::stdout().flush().unwrap();
        let temp1: String = read!("{}\n");

        match temp_re.captures(temp1.as_str()) {
            Some(x) => {
                let n = x.get(1).unwrap().as_str().parse::<f64>().unwrap();
                let unit = x.get(2).unwrap().as_str();
                match unit {
                    "f" | "F" => {
                        let c_temp = (n - 32.0) / 1.8;
                        break format!("{}C", c_temp);
                    }
                    "c" | "C" => {
                        let f_temp = n * 1.8 + 32.0;
                        break format!("{}F", f_temp);
                    }
                    _ => {}
                }
            }
            None => {
                println!("{} sucks. try again", temp1);
            }
        }
    };
    println!("{}", converted_temp);
}

fn nth_fibonacci() -> () {
    loop {
        print!("which Fibonacci number you want? ");
        io::stdout().flush().unwrap();
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("failed to read n");

        // let n: u128 = match n.trim().parse() {
        let n = match n.trim().parse::<u128>() {
            Ok(n) => n,
            Err(_) => {
                println!("{} sucks. try again", n.trim());
                continue;
            }
        };

        let mut f1: u128 = 0;
        let mut f2: u128 = 1;
        if n <= 1 {
            f2 = n;
        } else {
            for _ in 1..=(n - 1) {
                // println!("{:?}", (f1, f2));
                let f_new: u128 = f1 + f2;
                f1 = f2;
                f2 = f_new;
            }
        }
        println!("the {}th fibonacci number is {}", n, f2);
    }
}

fn main() {
    // ternary();
    // simple_loop();
    // loop_user_input();
    // temp_unit_convert();
    nth_fibonacci();
}
