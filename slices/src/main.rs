fn main() {
    let s = String::from("Lorem ipsum dolor sit amet");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{} {}", hello, world);

    let fw = first_word(&s);
    println!("First word: {}", fw);

    let sw = second_word(&s);
    println!("Second word: {}", sw);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return first_word(&s[i + 1..]);
        }
    }
    &s[..]
}
