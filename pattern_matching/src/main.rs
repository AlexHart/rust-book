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

    let funs: Vec<&dyn Fn() -> ()> = vec![
        &while_matching,
        &for_matching,
        &irrefutable_patterns,
        &matching_literals,
        &matching_named_variables,
        &multiple_patterns,
        &matching_range_of_values,
        &destructuring_matching,
        &destructuring_enums,
        &destructuring_nested_structs_and_enums,
        &ignoring_values_in_tuple,
        &ignoring_parts_of_value,
        &matching_guards,
        &binding_at_operator,
    ];

    for fun in &funs {
        fun();
    }

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

fn irrefutable_patterns(){
    let val: Option<i32> = None;
    if let Some(x) = val {
        println!("val = Some({})", x);
        ()
    }

    println!("Val equals None!");
    ()
}

fn matching_literals() {
    for x in 1..5 {
        match x {
            1 => println!("{} is one", x),
            2 => println!("{} is two", x),
            _ => println!("{} is anything", x),
        }
    }
}

fn matching_named_variables() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

fn multiple_patterns() {
    for x in -1..5 {
        match x {
            1 | 2 => println!("One or two"),
            3 => println!("Three"),
            _ => println!("anything ({})", x)
        }
    }
}

fn matching_range_of_values() {
    for x in 0..7 {
        match x  {
            1..=5 => println!("one through five ({})", x),
            _ => println!("Out of range ({})", x),
        }
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn destructuring_matching() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn destructuring_enums() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
    }
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn destructuring_nested_structs_and_enums() {
    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
        Message2::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }
}

fn ignoring_values_in_tuple() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }
}

fn ignoring_parts_of_value() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    // ignore it using ..
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    // other example with tuples
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
}

fn matching_guards() {
    for x in -2..8 {
        let num: Option<i32>;
        if  x == 0 {
            num = None;
        }else {
            num = Some(x);
        };

        match num {
            Some(1) => println!("ONE"),
            Some(x) if x < 5 || x == 6 => {
                println!("Less than five or equals 6: {}", x);
            },
            Some(x) => println!("{}", x),
            None => (),
        }
    }
}

enum Message3 {
    Hello { id: i32 },
}

fn binding_at_operator() {
    for i in 0..14 {
        let msg = Message3::Hello { id: i };

        match msg {
            Message3::Hello {
                id: id_variable @ 3..=7,
            } => println!("Found and an id in range 3..7: {}", id_variable),
            Message3::Hello { id: id_variable @ 10..=12 } => {
                println!("Found an id in range 10..12: {}", id_variable);
            },
            Message3::Hello { id } => println!("Found some other id: {}", id), 
        }
    }
}