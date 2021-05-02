fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("Number was something other than zero");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // loops.

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
        println!("{}\tAgain!", counter);
    };

    println!("The result is {}", result);

    counter = 3;
    while counter >= 0 {
        println!("While\t{}", counter);
        counter -= 1;
    }
    println!("While: LIFTOFF!!!");

    // For
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("The value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("for\t{}!", number);
    }
    println!("For: LIFTOFF!!!");
}
