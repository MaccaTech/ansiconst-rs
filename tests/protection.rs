mod common;
use common::check_fmt;

use ansiconst::*;

#[test]
fn test_protection() {
    check_fmt(
        "no ansi",
        styled_format_args!(Ansi::no_ansi(), "{}",
            styled_format_args!(Effect::Bold, Colour::Red, "no ansi")
        ).to_string()
    );
    check_fmt(
        "\x1B[1mBold-only\x1B[22m",
        styled_format_args!(Effect::Bold.only(), Effect::Italic, "Bold-only").to_string()
    );
    check_fmt(
        "\x1B[34mBlue still blue\x1B[39m",
        styled_format_args!(Colour::Blue.only(), "Blue {}",
            styled_format_args!(Colour::Red, "still blue")
        ).to_string()
    );
}
