fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    // compiler error: re-assignment of immutable variable
    // x = 6;
    // println!("The value of x is: {}", x);

    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is: {}", y);

    const MAX_STEP: u32 = 1000;
    println!("The value of MAX_STEP is: {}", MAX_STEP);

    // Variable shadowing
    let x = x + 2;
    let x = x * 2;
    println!("The value of x is: {}", x);
    // Shadow to change the type
    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces: {:?}", spaces);
}
