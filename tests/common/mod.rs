#![allow(dead_code)]

use std::str;

pub fn check_fmt<S: AsRef<str>>(expect: &str, got: S) {
    println!("{: <8}{}\n{: <8}{}", "Expect:", expect, "Got:", got.as_ref());
    assert_eq!(expect, got.as_ref());
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
