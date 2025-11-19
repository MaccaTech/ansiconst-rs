use std::fmt::Write;
use std::str::FromStr;
use std::time::Instant;
use ansiconst::{ansi, Ansi, styled_format, styled_format_args};

const CYCLES: usize = 100000;

#[test]
#[ignore]
fn test_performance() {
    let mut buffer: String = String::new();
    let plain_string = String::from_str("Red").unwrap();

    const RED_STR_SET:   &str = "\x1B[31m";
    const RED_STR_RESET: &str = "\x1B39m";
    let now = Instant::now();
    for _ in 0 .. CYCLES {
        writeln!(&mut buffer, "{}", format_args!("{RED_STR_SET}{}{RED_STR_RESET}", &plain_string)).unwrap();
        buffer.clear();
    }
    let elapsed = now.elapsed();
    println!("{:<20}{:.2?}", "plain:", elapsed);

    const RED_ANSI: Ansi = ansi!(Red);
    let now = Instant::now();
    for _ in 0 .. CYCLES {
        writeln!(&mut buffer, "{}", format_args!("{RED_ANSI}{}{RED_ANSI:#}", &plain_string)).unwrap();
        buffer.clear();
    }
    let elapsed = now.elapsed();
    println!("{:<20}{:.2?}", "ansi:", elapsed);

    let now = Instant::now();
    for _ in 0 .. CYCLES {
        writeln!(&mut buffer, "{}", styled_format_args!(Red, "{}", &plain_string)).unwrap();
        buffer.clear();
    }
    let elapsed = now.elapsed();
    println!("{:<20}{:.2?}", "styled_format_args:", elapsed);

    let now = Instant::now();
    for _ in 0 .. CYCLES {
        writeln!(&mut buffer, "{}", styled_format!(Red, "{}", &plain_string)).unwrap();
        buffer.clear();
    }
    let elapsed = now.elapsed();
    println!("{:<20}{:.2?}", "styled_format:", elapsed);
}
