pub struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    pub fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let next = self.a + self.b;
        self.a = self.b;
        self.b = next;

        Some(self.a) // Infinite iterator
    }
}

pub fn ex_4() {
    let fibs: Vec<u64> = Fibonacci::new().take(15).collect();

    println!("First 15 Fibonacci numbers: {:?}", fibs);
}