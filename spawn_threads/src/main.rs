use std::thread;
use std::time::Duration;

fn main() {

    let handle = thread::spawn(|| {
        for i in 1..11 {
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        } 
    });

    for i in 1..6 {
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    let v = vec![1, 2, 3];
    let f = move || {
        println!("Here's a vector: {:?}", v);
    };
    let handle = thread::spawn(f);

    //drop(v);
    handle.join().unwrap();


    // Thread that returns a value ---------------------------------
    let handle = thread::spawn(|| {
        let mut res = 0;
        for i in 1..1_000 {
            res += i;
        };
        res
    });
    let sum = handle.join().unwrap();
    println!("Value calculated in another thread: {}", sum);

    // Thread that returns a value and fails -----------------------
    println!("");
    let handle = thread::spawn(|| {
        let mut res = 0;
        for i in 1..1_000_000 {
            res += i;
        };
        res
    });
    match handle.join() {
        Ok(sum) => println!("Value calculated in another thread: {}", sum),
        Err(err) => println!("There was an error on the spawned thread: {:?}", err)
    }

    println!("\r\nProgram finished!");
}
