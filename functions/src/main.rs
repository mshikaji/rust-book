fn main() {
    println!("Hello, world!");

    another_function();
    yet_another_function(55);
    print_labeled_measurement(66, 'q');
    block_expression();

    let x = five();
    println!("The value of x is: {}", x);
}

fn another_function() {
    println!("Another function.");
}

fn yet_another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn block_expression() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("{}", y == 4);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}
