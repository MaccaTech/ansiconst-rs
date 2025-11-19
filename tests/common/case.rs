use super::expect::Expect;
use ansiconst::{ansi, Ansi};

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct TestCase {
    pub ansi:   Ansi,
    pub expect: Expect,
}

impl TestCase {
    pub fn all() -> [TestCase; 13] {
        [
            TestCase::new(ansi!(Bold)),
            TestCase::new(ansi!(Faint)),
            TestCase::new(ansi!(Italic)),
            TestCase::new(ansi!(Underline)),
            TestCase::new(ansi!(Blink)),
            TestCase::new(ansi!(Reverse)),
            TestCase::new(ansi!(Hidden)),
            TestCase::new(ansi!(Strike)),
            TestCase::new(ansi!(Red)),
            TestCase::new(ansi!(Blue.bg())),
            TestCase::new(ansi!(Ansi::empty())),
            TestCase::new(ansi!(Ansi::reset())),
            TestCase::new(ansi!(Ansi::no_ansi())),
        ]
    }

    const fn new(ansi: Ansi) -> Self {
        Self {
            ansi,
            expect: Expect::from_ansi(ansi),
        }
    }

    pub fn add(&self, other: Self) -> Self {
        Self {
            ansi:   self.ansi.add(other.ansi),
            expect: self.expect.add(other.expect),
        }
    }

    pub fn not(&self) -> Self {
        Self {
            ansi:   self.ansi.not(),
            expect: self.expect.not(),
        }
    }
}
