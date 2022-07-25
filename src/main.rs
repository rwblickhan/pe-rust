use std::iter::Iterator;

fn main() {
    println!("Hello, world!");
    println!("pe1: {}", pe1());
    println!("pe2: {}", pe2());
    println!("pe3: {}", pe3());
}

fn pe1() -> u64 {
    (0..1000).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}

struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new(first: u64, second: u64) -> Self {
        Fibonacci {
            a: first,
            b: second,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.b;
        self.b = self.a;
        self.a += r;
        Some(r)
    }
}

fn pe2() -> u64 {
    let fib = Fibonacci::new(0, 1);
    fib.filter(|x| x % 2 == 0)
        .take_while(|x| x < &4000000)
        .sum()
}

fn pe3() -> u64 {
    (2..600851475142)
        .filter(|x| 600851475143 % x == 0)
        .max()
        .unwrap()
}
