use std::io;

fn main() {
    let mut problems = String::new();
    io::stdin().read_line(&mut problems).expect("Error reading the number of problems");
    let problems:u16 = problems.trim().parse().expect("Expected a positive number");

    let mut solutions:u16 = 0;

    for _ in 0..problems {
        let mut suggestions = String::new();
        io::stdin().read_line(&mut suggestions).expect("Error reading the suggestions line");
        let suggestions = suggestions.trim().split(" ");
        let mut agreed = 0;

        for s in suggestions {
            let s:u8 = s.parse().expect("Not one or 0");
            if s == 1 {
                agreed += 1;
            }
        }

        if agreed > 1 {
            solutions += 1;
        }
    }

    println!("{}", solutions);
}
