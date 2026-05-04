struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        self.curr = self.next;
        self.next = current + self.next;

        // Note: This will wrap or panic on overflow 
        // depending on your build profile.
        Some(current)
    }
}

fn main() {
    let fib = Fibonacci::new();

    // Print the first 10 numbers
    for num in fib.take(10) {
        println!("{}", num);
    }
}
