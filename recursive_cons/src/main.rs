use crate::List::{Nil, Cons};

fn main() {
    let list = Cons(1, 
        Box::new(Cons(2, 
            Box::new(Cons(3, 
                Box::new(Cons(4, 
                    Box::new(Nil))))))));

    print_list(list);
}

fn print_list(list: List) {
    match list {
        Cons(x, l) => {
            println!("{}", x);
            print_list(*l);
        },
        Nil => println!("End of the list"),
    };
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}
