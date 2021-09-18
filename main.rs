// SCALAR TYPES
// Integer types and Floating point types

fn main() {
    let a = 2; // u8
    let b = 3; // i8

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
}

// Numeric operations

fn main() {
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;
}

// The Boolean type

fn main() {
    let t = true;
    let f: bool = false; // with explicit type annotation
}

// The Character type

fn main() {
    let c = 'z';
    let z = 'Ƶ';
    let heart_eyed_cat = '😻';
}

// COMPOUND TYPES
// The Tuple type

fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}

// To turn it into three separate variables: Destructuring

fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
}

// Access a tuple element without destructuring

fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}

// Array type (One type and fixed lenght)
fn main() {
let a = [1, 2, 3, 4, 5];
}
// Example
let months = ["January", "February", "March", "April", "May", "June", "July",
"August", "September", "October", "November", "December"];

// Accessing array elements
fn main() {
let a = [1, 2, 3, 4, 5];
let first = a[0];
let second = a[1];
}
// If we wanted access a[5] it would compile but it would also panic in runtime


