mod common;
use common::TestLines;

use ansiconst::{*, io::AnsiWrite};

use std::process::Command;
use std::str;

fn check_line<'a>(got: &'a str, expect: &'static str) {
    println!("{}", got);
    assert_eq!(got, expect);
}

#[test]
fn test_output_noansi() {
    let output = Command::new("cargo")
        .env("FORCE_COLOR", "1")
        .args(&["test", "test_noansi", "--quiet", "--", "--nocapture", "--include-ignored"])
        .output().unwrap();
    let stdout = str::from_utf8(&output.stdout).unwrap();
    let stderr = str::from_utf8(&output.stderr).unwrap();
    let mut stdout_lines = TestLines::new(stdout);
    let mut stderr_lines = TestLines::new(stderr);

    check_line(stdout_lines.next().unwrap(), "\x1B[1;31mansi\x1B[22;39m");
    check_line(stdout_lines.next().unwrap(), "\x1B[1;31mall ansi\x1B[22;39m");
    check_line(stdout_lines.next().unwrap(), "no ansi");
    check_line(stderr_lines.next().unwrap(), "\x1B[1;31mansi\x1B[22;39m");
    check_line(stderr_lines.next().unwrap(), "\x1B[1;31mall ansi\x1B[22;39m");
    check_line(stderr_lines.next().unwrap(), "no ansi");
}

#[test]
#[ignore = "used by output test"]
fn test_noansi() {
    // Stdout
    println!("[test_start]");
    paintln!(Effect::Bold, Colour::Red, "ansi");
    io::ansiout().all_ansi();
    assert!(io::ansiout().is_all_ansi());
    paintln!(Effect::Bold, Colour::Red, "all ansi");
    io::ansiout().no_ansi();
    assert!(io::ansiout().is_no_ansi());
    paintln!(Effect::Bold, Colour::Red, "no ansi");
    println!("[test_end]");

    // Stderr
    eprintln!("[test_start]");
    epaintln!(Effect::Bold, Colour::Red, "ansi");
    io::ansierr().all_ansi();
    assert!(io::ansierr().is_all_ansi());
    epaintln!(Effect::Bold, Colour::Red, "all ansi");
    io::ansierr().no_ansi();
    assert!(io::ansierr().is_no_ansi());
    epaintln!(Effect::Bold, Colour::Red, "no ansi");
    eprintln!("[test_end]");
}
