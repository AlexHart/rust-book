fn main() {

    let (x, y, z) = (1, 2, 3);
    println!("x={}\ty={}\tz={}", x, y, z);

    for i in 0..10 {
        if i == 0 {
            match_example(None);
        }
        else {
            match_example(Some(i));
        }
    }

    if_matching(33);
    if_matching(22);
    while_matching();
    for_matching();
}

fn match_example(val: Option<i32>) {
    match val {
        Some(x) => println!("It has a value: {}", x),
        None => println!("It doesn't have a value")
    }

}

fn if_matching(x: i32) {
    let val:Option<i32> = Some(33);

    println!("Value passed: {}", x);

    if let Some(val) = val {
        if x == val {
            println!("{} it's equal to 33", x);
        } else {
            println!("{} it's not equal to 33", x);
        }
    } else {
        println!("Val is None");
    }

}

fn while_matching() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn for_matching() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}