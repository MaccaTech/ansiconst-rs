mod common;
use common::TestLines;

use ansiconst::*;

use std::process::Command;
use std::str;

fn run_test(feature: Option<&'static str>, max_effect_size: usize, max_colour_size: usize, max_ansi_size: usize) {
    let mut cmd = Command::new("cargo");
    cmd.args(&["test", "test_sizes", "--quiet"]);
    if let Some(feature) = feature {
        cmd.args(&["--features", feature]);
    }
    cmd.args(&["--", "--nocapture", "--include-ignored"]);
    let output = cmd.output().unwrap();
    let stdout = str::from_utf8(&output.stdout).unwrap();
    let mut lines = TestLines::new(stdout);

    let got_effect_size = usize::from_str_radix(lines.next().unwrap(), 10).unwrap();
    let got_colour_size = usize::from_str_radix(lines.next().unwrap(), 10).unwrap();
    let got_ansi_size   = usize::from_str_radix(lines.next().unwrap(), 10).unwrap();

    println!("[feature = {}]", feature.unwrap_or("none"));
    println!("Effect = {: >2} bytes, expected <= {: >2} bytes", got_effect_size, max_effect_size);
    assert!(got_effect_size <= max_effect_size, "Effect size too big: {} bytes", got_effect_size);
    println!("Colour = {: >2} bytes, expected <= {: >2} bytes", got_colour_size, max_colour_size);
    assert!(got_colour_size <= max_colour_size, "Colour size too big: {} bytes", got_colour_size);
    println!("Ansi   = {: >2} bytes, expected <= {: >2} bytes", got_ansi_size,   max_ansi_size);
    assert!(got_ansi_size   <= max_ansi_size,   "Ansi size too big: {} bytes",   got_ansi_size);
}

#[test]
fn test_output_sizes() {
    run_test(None, 1, 1, 6);
    run_test(Some("ansi256"), 1, 2, 8);
    run_test(Some("rgb"), 1, 4, 12);
}

#[test]
#[ignore = "used by output test"]
fn test_sizes() {
    println!("[test_start]");
    println!("{}", std::mem::size_of::<Effect>());
    println!("{}", std::mem::size_of::<Colour>());
    println!("{}", std::mem::size_of::<Ansi>());
    println!("[test_end]");
}
