use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let beginning_message = r#"
            WELCOME TO THE GUESSING GAME!
        You will be guessing numbers up to a certain range.
        Choose your difficulty:
        1: Want to try it out: Number up to 10
        2: Just Bored: Number up to 100
        3: Want to relax: Number up to 1_000
        4: I need Help: Number up to 100_000_000
        5: I am a psycho: Number up to 10_000_000_000

    "#;

    println!("{}", beginning_message);

    let mut difficulty = String::new();

    io::stdin()
        .read_line(&mut difficulty)
        .expect("Error reading the difficulty line");

    let difficulty: i64 = match difficulty.as_str().trim() {
        "1" => 10,
        "2" => 100,
        "3" => 1_000,
        "4" => 100_000_000,
        "5" => 10_000_000_000,
        _ => {
            println!("I had high expactations for you ... You let me down!");
            println!("Procceeding with Baby Mode");
            10
        }
    };

    let computer_guess = rand::thread_rng().gen_range(0..=difficulty);

    game(&computer_guess);

}
fn game(computer_guess: &i64) {
    let mut tries = 0;

    loop {
        println!("Input your guess: ");

        // this is where we store the guess of the user
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading the guess line");

        // Shadowing the guess variable with a different type variable
        let guess: i64 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\n{} is not a number :p \n ", guess.trim());
                continue;
            }
        };

        match guess.cmp(&computer_guess) {
            Ordering::Less => {
                println!("Guess Higher");
                tries = tries + 1;
            }
            Ordering::Equal => {
                println!("After making {} mistakes, YOU WON, ", tries);
                break;
            }
            Ordering::Greater => {
                println!("Guess Lower");
                tries = tries + 1;
            }
        }
    }
}
