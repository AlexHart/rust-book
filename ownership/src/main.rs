fn main() {
    let s = String::from("Hello");
    let mut s2 = String::from(s);
    s2.push_str(", world!");
    println!("{}", s2);

    free_string_memory();
    free_string_memory_with_clone();

    let ss = String::from("hello");
    let s_res = takes_ownership(ss);
    println!("{}", s_res);

    let (ss2, len) = calculate_length(s_res);
    println!("The length of '{}' is {}.", ss2, len);
}

fn free_string_memory() {
    let s1 = String::from("Hello world 2!");
    let mut s2 = s1;

    s2.push_str("!!!");

    println!("{}", s2);
    // this can't be done because s2 borrowed s1
    //println!("{}", s1);
}

fn free_string_memory_with_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 =  {}, s2 = {}", s1, s2);
}

fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string);
    some_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
