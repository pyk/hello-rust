fn another_function() {
    println!("another_function function");
}

fn params(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn return_value(x: i32) -> i32 {
    return x + 5;
}

fn main() {
    println!("main function");
    another_function();
    params(2, 3);
    let o: i32 = return_value(5);
    println!("The value of o is: {}", o);
}
