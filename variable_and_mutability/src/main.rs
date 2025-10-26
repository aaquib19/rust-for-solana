fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; // This line would cause a compile-time error because x is immutable by default
    println!("The value of x is: {}", x);


    let mut y = 10;
    println!("The value of y is: {}", y);
    y = 15; // This is allowed because y is mutable
    println!("The value of y is: {}", y);



    let z = 20;
    {
        let z = z + 5; // This creates a new variable z that shadows the outer z
        println!("The value of inner z is: {}", z);
    }
    println!("The value of outer z is: {}", z);
    let z = 45; 
    println!("The value of new z is: {}", z);  


    let spaces = "   ";
    let spaces = spaces.len();
    println!("The number of spaces is: {}", spaces);

    let mut spaces_mut = "   ";
    // spaces_mut = spaces_mut.len()// This line would cause a compile-time error because spaces_mut is a &str, not a usize
    println!("The number of spaces is: {}", spaces_mut);
}
