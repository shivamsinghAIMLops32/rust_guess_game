use std::io;
use rand::Rng;
fn main() {
    println!("Welcome to guessing number game!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Secret number is: {}", secret_number);
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Error in reading user input");
    println!("You guessed: {}", guess);

    while guess.trim() != secret_number.to_string().trim() {
        // if guessed number is closer to the secret number, print close or else far also
        if guess.trim().parse::<i32>().unwrap() > secret_number {
            println!("Your guess is too high!");
        } else {
            println!("Your guess is too low!");
        }
        guess.clear();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error in reading user input");
        println!("You guessed: {}", guess);
    }
    println!("Congratulations! You guessed the number!");
}
