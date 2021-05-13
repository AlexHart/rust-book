//! As referenced here: https://doc.rust-lang.org/book/ch15-04-rc.html

use std::rc::Rc;
use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Rc<List>),
    Nil
}

fn print_list(list: &List) {
    match list {
        Cons(x, l) => {
            println!("{}", x);
            print_list(&*l);
        },
        Nil => println!("End of the list"),
    };
}

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
        print_list(&c);
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    print_list(&b);

    println!("End of the program...");
}

impl Drop for List {
    fn drop(&mut self) { 
        match &self { 
            Cons(x, _) => println!("Out of scope {}", x),
            Nil => println!("Out of scope Nil"), 
        }
    }
}