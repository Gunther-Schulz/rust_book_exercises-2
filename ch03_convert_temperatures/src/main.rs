fn main() {
    let f: [i32; 6] = [0, 10, 20, 30, 40, 50];
    for element in f.iter() {
        to_f(element);
    }
    // TODO: F -> C
}

fn to_f(degree: &i32) {
    let x = (*degree * 9 / 5) + 32;
    println!("The value of {} Celsius in Fahrenheit is: {}", *degree, x);
}
