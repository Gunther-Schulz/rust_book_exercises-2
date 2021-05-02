fn largest_return_ref<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for (i, _) in list.iter().enumerate() {
        if list[i] > *largest {
            largest = &list[i];
        }
    }
    largest
}

fn largest_clone<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();
    for mut item in list {
        item = item;
        if item > &largest {
            largest = item.clone();
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_clone(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_return_ref(&char_list);
    println!("The largest char is {}", result);
}
