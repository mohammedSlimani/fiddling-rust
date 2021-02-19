use std::io;
const TOO_LONG: usize = 10;

fn main() {
    let mut inputs = String::new();
    io::stdin()
        .read_line(&mut inputs)
        .expect("Error reading the number of inputs");
    let inputs: usize = inputs.trim().parse().expect("Expected a positive number");

    for _ in 0..inputs {
        let mut word = String::new();
        io::stdin()
            .read_line(&mut word)
            .expect("Error reading the word line");
        let word = word.trim();

        if word.len() <= TOO_LONG {
            println!("{}", word);
        } else {
            let first_char = word.chars().nth(0).unwrap();
            let last_char = word.chars().nth(word.len() - 1).unwrap();
            println!("{}{}{}",first_char, word.len() -  2, last_char );
        }
    }
}
