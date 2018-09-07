use std::thread;
use std::time::Duration;
use std::collections::HashMap;

// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

// fn calculate_mode(vec: &Vec<i32>) -> i32 {
//     let mut map = HashMap::new();
//     let mut high_count = 0;
//     let mut mode = vec[0];

//     for num in vec {
//         let count = map.entry(num).or_insert(0);
//         *count += 1;
//         // or above two lines could be swapped with this instead
//         // map.entry(num).and_modify(|count| {*count += 1}).or_insert(1);

//         if count > &mut high_count {
//             high_count = *count;
//             mode = *num;
//         }
//     }

//     mode
// }

// memonization with a Cacher taking generic type of closure T
// and caching values in a HashMap of u32: u32's
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    values: HashMap<u32, u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.values.get(&arg) {
            Some(&v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            },
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_closure = |num| {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };
    // and call below with `expensive_closure(intensity)`
    // or use Cacher with closure as in this example

    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        // next call will be cached
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
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

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

#[test]
fn call_cacher_with_different_values() {
    // closure here is simply `let closure = |a| { a }`
    let mut c = Cacher::new(|a| a);

    let _v1 = c.value(1);
    let _v2 = c.value(2);
    let v3 = c.value(3);

    // this works OK now since we're using a HashMap
    assert_eq!(v3, 3);
}
