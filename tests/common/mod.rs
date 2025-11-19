#![allow(dead_code)]

pub mod case;
pub mod expect;
use std::str;

#[macro_export]
macro_rules! assert_eq_print {
    // Base case:
    ($x:expr, $y:expr $(,)?) => {
        println!("{: <8}{}\n{: <8}{}", "Expect:", $x, "Got:", $y);
        assert_eq!($x, $y)
    };
    // Internal:
    (GOT_MORE: $x:expr, $y:expr $(,)?) => {
        println!("{: <8}{}", "Got:", $y);
        assert_eq!($x, $y)
    };
    // Recurse internal:
    (GOT_MORE: $x:expr, $y:expr, $($z:tt)+) => {
        assert_eq_print!(GOT_MORE: $x, $y);
        assert_eq_print!(GOT_MORE: $x, $($z)+);
    };
    // Recurse:
    ($x:expr, $y:expr, $($z:tt)+) => {
        assert_eq_print!($x, $y);
        assert_eq_print!(GOT_MORE: $x, $($z)+);
    };
}

pub struct TestLines<'a> {
    lines: Option<str::Lines<'a>>,
}

impl<'a> TestLines<'a> {
    pub fn new(output: &'a str) -> Self {
        let mut lines = output.lines();
        loop {
            let line = lines.next().unwrap();
            if line == "[test_start]" { break; }
        }
        Self { lines: Some(lines) }
    }

    pub fn next<'b>(&'b mut self) -> Option<&'a str> where 'a: 'b {
        if let Some(lines) = self.lines.as_mut() {
            let line = lines.next().unwrap();
            if line == "[test_end]" {
                self.lines = None;
            } else {
                return Some(line);
            }
        }
        None
    }
}
