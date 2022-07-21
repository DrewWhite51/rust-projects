// Importing from standard library: input/output
use std::io;
use rand::Rng;


fn main() {
    println!("Guess the number!");
    // Getting user input from the user as a string
    let mut first_guess = String::new();
    let mut second_guess = String::new();    

    let secret_number = rand::thread_rng().gen_range(1..=100);




    println!("Your first guess is: ");
    io::stdin().read_line(&mut first_guess).expect("Failed to read the line");
    println!("Your second guess is: ");
    io::stdin().read_line(&mut second_guess).expect("Failure to read second guess.");

    println!("You guessed {} the first time and {} the second time!", first_guess,second_guess);
    println!("The secret number is {}", secret_number);


}
