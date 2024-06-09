mod common;
use common::check_fmt;

use ansiconst::*;

#[test]
fn test_macros() {
    check_fmt("Plain \x1B[31mRed\x1B[39m Plain",          format!("Plain {} Plain", styled_format_args!(Colour::Red, "Red")));
    #[cfg(feature="ansi256")]
    check_fmt("Plain \x1B[38;5;128mPurple\x1B[39m Plain", format!("Plain {} Plain", styled_format_args!(Colour::Ansi256(128), "Purple")));
    check_fmt("Plain \x1B[41mRed\x1B[49m Plain",          format!("Plain {} Plain", styled_format_args!(Colour::Red.bg(), "Red")));
    #[cfg(feature="ansi256")]
    check_fmt("Plain \x1B[48;5;128mPurple\x1B[49m Plain", format!("Plain {} Plain", styled_format_args!(Colour::Ansi256(128).bg(), "Purple")));
    check_fmt("Plain \x1B[1mBold\x1B[22m Plain",          format!("Plain {} Plain", styled_format_args!(Effect::Bold, "Bold")));
    check_fmt("Plain \x1B[2mFaint\x1B[22m Plain",         format!("Plain {} Plain", styled_format_args!(Effect::Faint, "Faint")));
    check_fmt("Plain \x1B[3mItalic\x1B[23m Plain",        format!("Plain {} Plain", styled_format_args!(Effect::Italic, "Italic")));
    check_fmt("Plain \x1B[4mUnderline\x1B[24m Plain",     format!("Plain {} Plain", styled_format_args!(Effect::Underline, "Underline")));
    check_fmt("Plain \x1B[5mBlink\x1B[25m Plain",         format!("Plain {} Plain", styled_format_args!(Effect::Blink, "Blink")));
    check_fmt("Plain \x1B[7mReverse\x1B[27m Plain",       format!("Plain {} Plain", styled_format_args!(Effect::Reverse, "Reverse")));
    check_fmt("Plain \x1B[8mHidden\x1B[28m Plain",        format!("Plain {} Plain", styled_format_args!(Effect::Hidden, "Hidden")));
    check_fmt("Plain \x1B[9mStrike\x1B[29m Plain",        format!("Plain {} Plain", styled_format_args!(Effect::Strike, "Strike")));
    check_fmt(
        "Plain \x1B[1mBold-only \x1B[4mBoth Bold & Underline\x1B[24m Bold-only again\x1B[22m Plain",
        format!("Plain {} Plain", styled_format_args!(Effect::Bold, "Bold-only {} Bold-only again",
            styled_format_args!(Effect::Underline, "Both Bold & Underline")
        ))
    );
    check_fmt(
        "Plain \x1B[1mBold \x1B[22;3mItalic\x1B[23;1m Bold\x1B[22m Plain",
        format!("Plain {} Plain", styled_format_args!(Effect::Bold, "Bold {} Bold", styled_format_args!(Effect::Bold.not(), Effect::Italic, "Italic")))
    );
    check_fmt(
        "Plain \x1B[33mYellow \x1B[31mRed\x1B[33m Yellow\x1B[39m Plain",
        format!("Plain {} Plain", styled_format_args!(Colour::Yellow, "Yellow {} Yellow", styled_format_args!(Colour::Red, "Red")))
    );
    check_fmt(
        "Plain \x1B[1mBold \x1B[2mFaint\x1B[22;1m Bold\x1B[22m Plain",
        format!("Plain {} Plain", styled_format_args!(Effect::Bold, "Bold {} Bold", styled_format_args!(Effect::Faint, "Faint")))
    );
    check_fmt(
        "Plain \x1B[2mFaint \x1B[1mBold\x1B[22;2m Faint\x1B[22m Plain",
        format!("Plain {} Plain", styled_format_args!(Effect::Faint, "Faint {} Faint", styled_format_args!(Effect::Bold, "Bold")))
    );
}
