
pub struct Guess {
    value : u32
}

impl Guess {
    pub fn new(value: u32) -> Guess{
        Guess { value }
    }
    pub fn value(&self) -> u32 {
        self.value
    }
}

pub  fn test() {
    let g = Guess::new(30);
    println!("{:?}", g.value());
}

