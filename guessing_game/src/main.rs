use std::io;// for input and output

use rand::Rng;// for random number generation
use std::cmp::Ordering;// to compare two values

fn main() {
    println!("Gues the number!");

   


    let secret_number = rand::thread_rng().gen_range(1..=100);

     println!("Please input your guess.");
    let mut guess = String::new();// making it mutable to store user input
    

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");//u32 is unsigned 32 bit integer

    println!("You guessed : {guess}");
    println!("The secret number is:{}", secret_number);

    match guess.cmp(&secret_number){//comparing guess with secret number
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

}
