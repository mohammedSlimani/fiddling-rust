use std::env;
use std::convert::TryInto;

fn main() {
    let args: Vec<String> = env::args().collect();
    let star_size : u32 = match args.get(1){
        Some(star_size) => {
            star_size.parse().expect("Please give a valid positive number")
        },
        None => {
            println!("You didn't provide any argument, using 10");
            10
        }
    };

    let star_symbol: &str = match args.get(2) {
        Some(symbol) => {symbol},
        None => {
            println!("You didn't provide any second argument for star symbol, using * ");
            "*"
        }
    };

    for x in 0..star_size {
        let repeated = star_symbol.repeat(x.try_into().unwrap());
        println!("{}", repeated);
    }
}
