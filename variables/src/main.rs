const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn reassign() {
    // x must be mutable if we want to reassign it later
    let mut x = 5;
    println!("The value of x is: {}", x);

    // the following line will break compilation unless x is mutable
    x = 6;

    println!("The value of x is: {}", x);
    println!("3hrs is {}secs", THREE_HOURS_IN_SECONDS)
}

fn shadowing() {
    let x = 5;
    // use let to shadow x and set it to 5 + 1
    let x = x + 1;

    {
        // use let to shado x in the inner scope and set it to 6 * 2
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    // we're back in the outer scope where is 6
    println!("The value of x in the outer scope is: {}", x);

    // shadowing can also allow a change of type for a given variable
    let ss = "foo";
    println!("ss originally set to string {}", ss);
    let ss = ss.len();
    println!("ss shadowed and to int {}", ss);

    // mutability cannot be used for type change
    let mut sss = "bar";
    /*
    the following line will not compile because it's an attempt to change the
    type of sss without shadowing
    */
    // sss = sss.len();
    // reassigning sss is ok as long as you don't change the type
    sss = "baz";
    // the compiler issues a warning (FTW!) because the original value of sss (bar) was never used
    println!("sss is mutable, was bar and now is {}", sss);
}

fn main() {
    reassign();
    shadowing();
}
