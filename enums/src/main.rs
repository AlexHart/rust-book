fn main() {
    let p1 = Person::Employee(50_000);
    let p2 = Person::Boss("IT".to_owned());

    println!("{:?}", &p1);
    println!("{:?}", &p2);

    let msg = Message::Move { x: 10, y: 20 };
    println!("{:?}", msg);

    match msg {
        Message::Move { x, y } => println!("x: {}\ty: {}", x, y),
        _ => println!("No match"),
    };

    if let Message::Move { x, y } = msg {
        println!("x: {}\ty: {}", x, y);
    }
}

#[derive(Debug)]
enum Person {
    Boss(String),
    Employee(u32),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
