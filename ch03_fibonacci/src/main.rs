fn main() {
    let nth: [i32; 6] = [1, 10, 20, 30, 40, 50];
    for element in nth.iter() {
        println!("{}", fibo(element));
    }
}

fn fibo(nth: &i32) -> u64 {
    let mut fib_a: [u64; 3] = [0, 1, 1];
    let mut idx = 3;
    let mut result = 0;
    if *nth <= 3 {
        result = fib_a[*nth as usize + 1];
    } else {
        while idx < *nth {
            fib_a[0] = fib_a[1];
            fib_a[1] = fib_a[2];
            fib_a[2] = fib_a[0] + fib_a[1];
            result = fib_a[2];
            idx += 1;
        }
    }
    result
}
