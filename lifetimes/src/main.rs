fn main() {
    one();

    let res = longest("Hello world", "this is a test with a long string");
    println!("{}", res);

    two();
    three();
}

fn one() {
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r: {}", r);

    let x = 5;
    let r = &x;
    println!("r: {}", r);
}

fn two() {
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is `{}`", result);
    }
}

fn three() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    {
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is `{}`", result);
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
