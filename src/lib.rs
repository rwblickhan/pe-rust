use std::{collections::HashSet, iter::Iterator};

pub struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    pub fn new(first: u64, second: u64) -> Self {
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
