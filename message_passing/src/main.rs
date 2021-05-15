use std::time::Duration;
use std::sync::mpsc;
use std::thread;
use std::sync::mpsc::TryRecvError::{Empty, Disconnected};

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in 1..11 {
            let val = format!("{} - Hello World!", i);
            match tx.send(val) {
                Ok(_) => (),
                Err(err) => { 
                    println!("Error sending {}", err);
                    panic!("Error sending, kill app");
                }
            }
        }
    });

    // Blocking receive
    // loop {
    //     match rx.recv() {
    //         Ok(received) => println!("Got: {}", received),
    //         Err(err) => {
    //             println!("Error: {}", err);
    //             break;
    //         }
    //     }
    // }

    // try_recv
    loop {
        match rx.try_recv() {
            Ok(received) => println!("Got {}", received),
            Err(e) => {
                match e {
                    Empty => continue,
                    Disconnected => break
                }
            }
        }
        thread::sleep(Duration::from_millis(64));
    }

    println!();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    println!();

    println!("Multiple producers example");
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    let tx2 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    thread::spawn(move || {
        for i in 1..11 {
            tx2.send(i.to_string()).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

}
