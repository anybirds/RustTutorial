use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Secret number is {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read a line.");

    let guess: u32 = guess.trim().parse().expect("Guess should be a number."); 
    
    println!("You guessed: {}", guess);
    
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Less"),
        Ordering::Greater => println!("Greater"),
        Ordering::Equal => println!("Equal")
    }

}    
