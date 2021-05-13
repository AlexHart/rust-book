use std::ops::Deref;

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    println!("x: {}", x);
    println!("y: {}", *y);

    let m = MyBox::new(String::from("Rust"));
    // pass the MyBox where it expects a &str
    hello(&m);
    // What the line on top really does
    hello(&(*m)[..]);
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {    
    type Target = T;
    
    fn deref(&self) -> &T { 
        &self.0
    }
}