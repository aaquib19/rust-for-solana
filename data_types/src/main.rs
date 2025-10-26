use std::io;
fn main() {
    let guess : u32 = "42".parse().expect("Not a number!");
    println!("The guessed number is: {}", guess);

    let sum = 5+10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5/3; // Results in -1
    // remainder
    let remainder = 43 % 5;
    println!("Sum: {}, Difference: {}, Product: {}, Quotient: {}, Truncated: {}, Remainder: {}", 
             sum, difference, product, quotient, truncated, remainder);

    let x : (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("Tuple values: {}, {}, {}", five_hundred, six_point_four, one);

    let a = [1,2, 3, 4, 5];
    let mut index = String:: new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Index entered was not a number");
    let element = a[index];
    println!("The value of the element at index {} is: {}", index, element);    
}
