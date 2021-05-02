use std::io;

fn main() {
    println!("Enter your Fizz buzz max number: ");
    
    let mut max = String::new();

    io::stdin()
            .read_line(&mut max)
            .expect("Failed to read line.");

    let max: u32 = max.trim().parse()
        .expect("Expecting a number");

    for i in 1..max {
        match i {
            x if { x % 15 == 0 } => {
                print!("Fizz Buzz");
            },
            x if { x % 3 == 0 } => {
                print!("Fizz");
            },
            x if { x % 5 == 0 } => {
                print!("Buzz");
            },
            _ => print!("{}", i)
        }

        if i < max-1 {
            print!(", ");
        }
    }

}
