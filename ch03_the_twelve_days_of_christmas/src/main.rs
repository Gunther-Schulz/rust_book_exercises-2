fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelvth",
    ];
    let gifts = [
        "Twelve drummers drumming,",
        "eleven pipers piping,",
        "Ten lords a leaping,",
        "nine ladies dancing,",
        "eight maids a milking,",
        "Seven swans a swimming,",
        "six geese a laying,",
        "five gold rings,",
        "Four calling birds,",
        "three French hens,",
        "Two turtle doves and",
        "a partridge in a pear tree",
    ];

    let mut combined: String = "".to_string(); // or to_owned()
    let mut idx = 0;
    for gift in gifts.iter().rev() {
        let g = gift.to_string();
        combined = g + " " + &combined;
        println!(
            "On the {} day of Christmas my true love gave to me {}",
            days[idx], combined
        );
        idx += 1;
    }
}
