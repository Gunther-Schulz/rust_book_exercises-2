use std::collections::HashMap;
use std::io::{stdin, stdout, Write};

#[derive(Debug, PartialEq)]
enum NextAction {
    Quit,
    Continue,
}

fn process_input(employees: &mut HashMap<String, String>) -> NextAction {
    let mut input = String::new();
    let fail_string = "I did not understand, try again.";
    print!("Please enter some text (\"quit\" to exit): ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut input)
        .expect("Did not enter a correct string");
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }
    let words = input.split_whitespace();
    let words_vec: Vec<&str> = words.collect();
    let count = words_vec.iter().count();
    let m = match count {
        4 => 4,
        1 => 1,
        _ => 0,
    };
    if m == 4 {
        if words_vec[0] == "add" && words_vec[2] == "to" {
            employees.insert(String::from(words_vec[1]), words_vec[3].to_string());
        } else {
            println!("{}",fail_string);
        }
    } else if m == 1 {
        if words_vec[0] == "list" {
            for (key, value) in employees {
                println!("Name: {} | Department: {}", key, value);
            }
        } else if words_vec[0] == "quit" {
            return NextAction::Quit;
        } else {
            println!("{}",fail_string);
        }
    } else {
        println!("{}",fail_string);
    }
    return NextAction::Continue;
}

fn main() {
    let mut employees: HashMap<String, String> = HashMap::new();
    loop {
        let res = process_input(&mut employees);
        if res == NextAction::Quit {
            break;
        }
    }
}
