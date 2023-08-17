
pub mod top {
    pub mod types {
        #[derive(Debug)]
        pub enum Colors {
            Red, Green, Blue, White, Black
        }
    }
    pub fn test() {
        use super::top::types::Colors;
        let v = vec![
            Colors::Red,
            Colors::Green,
            Colors::Blue
        ];
        println!("{:?}", v);
    }
}
