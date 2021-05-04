use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
//
// is equivalent to:
//
//     fn some_function<T, U>(t: &T, u: &U) -> i32
//         where T: Display + Clone,
//               U: Clone + Debug
//     {
struct Cacher<T, V, R>
where
    T: Fn(V) -> R,
    V: std::cmp::Eq + std::hash::Hash + Clone,
    R: Copy,
{
    calculation: T,
    values: HashMap<V, R>,
}

impl<T, V, R> Cacher<T, V, R>
where
    T: Fn(V) -> R,
    V: std::cmp::Eq + std::hash::Hash + Clone,
    R: Copy,
{
    fn new(calculation: T) -> Cacher<T, V, R> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: V) -> R {
        if self.values.contains_key(&arg) {
            self.values[&arg]
        } else {
            let v = (self.calculation)(arg.clone());
            self.values.insert(arg, v);
            v
        }
    }
}
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|val| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        val
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

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value("hello");
    let v2 = c.value("2");

    assert_eq!(v2, "2");
    assert_eq!(v1, "hello");
}
