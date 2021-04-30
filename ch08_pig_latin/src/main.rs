fn main() {
    let vowles = "aeiou";
    let vowles_vec: Vec<char> = vowles.chars().collect();
    let text = String::from("butter oaf");
    let words = text.split(" ");
    let words_vec: Vec<&str> = words.collect();

    let mut pl_text = "".to_string();
    for word in words_vec {
        let mut word_vec: Vec<char> = word.chars().collect();
        if vowles_vec.contains(&word_vec[0]) {
            word_vec.push('h');
            word_vec.push('a');
            word_vec.push('y');
        } else {
            word_vec.rotate_left(1);
            word_vec.push('a');
            word_vec.push('y');
        }
        let s: String = word_vec.iter().collect();
        pl_text = pl_text + " " + &s;
    }
    println!("s is {}", pl_text);
}
