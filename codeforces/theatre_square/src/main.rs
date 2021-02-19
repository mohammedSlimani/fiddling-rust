use std::io;

fn main() {

    let mut inputs: Vec<f64> = Vec::new();
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Error reading the number of inputs");
    let buffer = buffer.trim().split_whitespace();

    for i in buffer {
        let parsed:f64 = i.parse().unwrap();
        inputs.push(parsed);
    }

    let n = inputs[0];
    let m = inputs[1];
    let a = inputs[2];
    let solution:u64 = ((n/a).ceil() * (m/a).ceil()) as u64;

    println!("{}", solution);
}
