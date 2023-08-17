
pub fn test1() {
    let a = "Dave";
    let b = &a[1..2];
    println!("{}, {}", a, b);
}

// Demonstrates Lifetime Annotations
// Source: TRPL pp 191
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}

pub fn test_longest_string() {
    let x = String::from("abcd");
    let y = "xyz";
    let n = longest(&x.as_str(), &y);
    println!("Longest string is {}", n);
}
