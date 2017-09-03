#[allow(unused_variables)]
fn main() {
    println!("Data types");

    // Scalar type
    // Integers
    let int: i32 = 10;

    // Floating point 
    let f1: f32 = 0.0002;
    let f2: f64 = 0.000000000000001;

    // Numeric Ops
    let add = 5 + 2;
    let sub = 5 - 2;
    let mul = 5 * 2;
    let div = 5 / 2;
    let rem = 5 % 2; // remainder

    // Boolean type
    let is_work: bool = true;

    // Char type
    let c: char = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat: char = '😻';

    // Compound type
    // Tuple
    let tup: (i32, char, f32) = (1, 'a', 0.5);
    // destructuring tuple
    let (i, c, f) = tup;
    // or
    let i = tup.0;
    let c = tup.1;
    let f = tup.2;

    // Array
    let months = ["January", "February", "March", "April", "May", "June", 
                  "July", "August", "September", "October", "November", 
                  "December"];
    let jan = months[0];
    
}
