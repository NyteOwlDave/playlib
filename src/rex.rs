
// https://docs.rs/regex/latest/regex/

use regex::Regex;

pub fn test1(date: &str) {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Testing: {date}");
    if re.is_match(date) {
        println!("This is a date");
    } else {
        println!("This is not a date");
    }
}

