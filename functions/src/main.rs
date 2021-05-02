fn main() {
    println!("Hello, world!");

    for i in 1..11 {
        another_function(i);
    }

    let res = add(2, 2);
    println!("Add: {}", res);

    let y =  {
        let x = 3;
        x * x
    };
    println!("y: {}", y);

    println!("1 + 1: {}", plus_one(1));
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn plus_one(x: i32) -> i32 { 
    x + 1 
}