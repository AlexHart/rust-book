const MAX_POINTS: u32 = 100_000;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    // Shadowing.
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    // Shadowing from string to u32.
    let spaces = "      ";
    let spaces = spaces.len();
    println!("Number of spaces: {}", spaces);

    // Tuples.
    let tup: (i32, f64, char) = (1234, 56789.0, 'A');
    println!("With direct access: {} {} {}", tup.0, tup.1, tup.2);
    let (a, b, c) = tup;
    println!("With destructuring: {} {} {}", a, b, c);

    // Arrays.
    let arr = [1, 2, 3, 4, 5];
    println!("Print first array:");
    for i in 0..arr.len() {
        println!("{}", arr[i]);
    }

    println!("Print second array:");
    let arr2 = [1; 10]; // Array with 10 ones.
    for i in 0..arr2.len() {
        println!("{}", arr2[i]);
    }
}
