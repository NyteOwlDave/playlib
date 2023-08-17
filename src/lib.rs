
// https://betterprogramming.pub/how-to-structure-unit-tests-in-rust-cc4945536a32

pub mod first;
pub mod hilo;
pub mod slicers;
pub mod traffic;
pub mod messages;
pub mod hashers;
pub mod rex;

pub use first::top::types;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub fn test_vec() {
    let mut v = vec![1, 2, 3, 4, 5];
    for a in &mut v {
        *a += 1;
        println!("{}", a);
    }
    println!("{:?}", v)
}
