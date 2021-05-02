use core::hash::Hash;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specific_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specific_value, simulated_random_number);

    capturing_environment_with_closures();
    closure_with_move();
}

fn capturing_environment_with_closures() {
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
    println!("y {} == x {}? {:?}", y, x, equal_to_x(y));
}

fn closure_with_move() {
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

struct Cacher<T, Y, U>
where
    T: Fn(Y) -> U,
{
    calculation: T,
    stored_values: HashMap<Y, U>,
}

impl<T, Y, U> Cacher<T, Y, U>
where
    T: Fn(Y) -> U,
    Y: Hash + Eq + Copy,
    U: Copy,
{
    fn new(calculation: T) -> Cacher<T, Y, U> {
        Cacher {
            calculation,
            stored_values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: Y) -> U {
        match self.stored_values.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.stored_values.insert(arg, v);
                v
            }
        }
    }
}
