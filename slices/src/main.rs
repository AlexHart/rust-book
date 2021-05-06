fn main() {
    let s = String::from("Lorem ipsum dolor sit amet");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{} {}", hello, world);

    let fw = first_word(&s);
    println!("First word: {}", fw);

    let sw = second_word(&s);
    println!("Second word: {}", sw);

    // Vec slices examples
    let nums: Vec<i32> = vec!(1, 2, 3, 45, 234, 123);
    let nums_slice = &nums[2..4];
    print_nums_slice(nums_slice);
    
    let nums_slice = &nums[3..];
    print_nums_slice(nums_slice);

    let nums_slice = &nums[..5];
    print_nums_slice(nums_slice);
}

fn print_nums_slice(nums_slice: &[i32]){
    println!("--------------------------------");
    println!("Print slice of numbers with length: {}", nums_slice.len());
    for num in nums_slice {
        println!("Num in slice: {}", num);
    }
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
