use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    for i in 1..10 {
        v.push(i);
    }

    print_jump();
    for x in &v {
        println!("{}", x);
    }

    print_jump();

    if let Some(val) = v.get(0) {
        println!("The value caught from index 0 is {}", val);
    }

    print_jump();

    let v2 = vec![30, 40, 87];
    for x in &v2 {
        println!("{}", x);
    }

    print_jump();

    let cadenas = vec!["Здравствуйте", "Hola mundo! 😁✌", "नमस्ते"];
    for cadena in cadenas {
        println!("Esta es la cadena de texto: {} - {}", cadena, cadena.len());
        print_chars_from_string(cadena.to_owned());
        print_jump();
    }

    let mut dic = HashMap::new();
    dic.insert(1, "uno");
    dic.insert(2, "dos");
    dic.insert(3, "tres");
    if dic.contains_key(&4) == false {
        dic.insert(4, "cuatro");
    }
    for (k, v) in dic {
        println!("key: {}\tvalue: {}", k, v);
    }

    // Queues
    let mut q = VecDeque::new();
    q.push_back("hola");
    q.push_back("mundo");
    q.push_back("desde");
    q.push_back("Rust");
    println!("Queue length: {}", q.len());
    for _ in 0..q.len() {
        if let Some(popped) = q.pop_front() {
            println!("{}", popped);
        }
    }
    // while q.len() > 0 {
    //     if let Some(popped) = q.pop_front() {
    //         println!("{}", popped);
    //     }
    // }
    println!("Queue length: {}", q.len());
}

fn print_jump() {
    println!("----------");
}

fn print_chars_from_string(s: String) {
    for c in s.chars() {
        println!("{}", c);
    }
}
