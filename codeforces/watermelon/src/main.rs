use std::io;

fn main() {
    let mut weight = String::new();
    io::stdin().read_line(&mut weight).expect("Error reading the weight");
    let weight:u8 = weight.trim().parse().expect("Weight is not a number");

    if weight < 4 || weight % 2 != 0 {
        println!("NO")
    } else {
        println!("YES")
    }

}
