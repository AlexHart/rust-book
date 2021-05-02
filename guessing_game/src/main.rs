use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let mut rng = rand::thread_rng();

    let secret_number = rng.gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    let mut tries = 1;
    loop {
        if tries >= 4
        {
            println!("Sorry, you tried too many times. Goodbye.");
            return;
        }

        println!("Please input your guess. Tries: {}", tries);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Couldn't parse that to a number");
                tries += 1;
                continue;
            },
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };      

        tries += 1;
    }
}
