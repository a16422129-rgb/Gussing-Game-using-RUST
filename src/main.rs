use std::io;

fn main() {
    println!("Guess the number!");
    println!("please input the guess!!");

    let mut guess: String = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read your guess");

    println!("You Guessed {guess}");
    

}
