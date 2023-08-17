
use std::collections::HashMap;

pub fn test1() {
    let names = vec![String::from("Dave"),
                        String::from("wellsted")];
    let values = vec![1964, 2022];
    let map: HashMap<_,_> = names.iter().zip(values.iter()).collect();
    for (n, v) in map {
        println!("{} = {}", n, v);
    }
}

pub fn test2() {
    println!("See TRPL pp 146");
}

pub fn frequency() {

    let vec: Vec<i32> = vec![1, 2, 3, 1, 1, 2];
    let mut hash: HashMap<i32, i32> = HashMap::new();
    for n in vec.iter() {
        if let Some(m) = hash.get(&n) {
            hash.insert(*n, m+1);
        } else {
            hash.insert(*n, 1);
        }
    }
    let mut max_value = 0i32;
    let mut max_count = 0i32;
    for (n, m) in hash.iter() {
        if *m > max_count {
            max_value = *n;
            max_count = *m;
        }
    }
    println!("Max Value = {}", max_value);
    println!("Max Count = {}", max_count);

}
