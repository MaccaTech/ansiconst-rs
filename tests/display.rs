mod common;
use common::check_fmt;

use ansiconst::*;

#[test]
fn test_display() {
    check_fmt("Plain \x1B[31mRed\x1B[39m Plain",          format!("Plain {ansi}Red{ansi:#} Plain",       ansi=Colour::Red));
    #[cfg(feature="ansi256")]
    check_fmt("Plain \x1B[38;5;128mPurple\x1B[39m Plain", format!("Plain {ansi}Purple{ansi:#} Plain",    ansi=Colour::Ansi256(128)));
    check_fmt("Plain \x1B[41mRed\x1B[49m Plain",          format!("Plain {ansi}Red{ansi:#} Plain",       ansi=Colour::Red.bg()));
    #[cfg(feature="ansi256")]
    check_fmt("Plain \x1B[48;5;128mPurple\x1B[49m Plain", format!("Plain {ansi}Purple{ansi:#} Plain",    ansi=Colour::Ansi256(128).bg()));
    check_fmt("Plain \x1B[1mBold\x1B[22m Plain",          format!("Plain {ansi}Bold{ansi:#} Plain",      ansi=Effect::Bold));
    check_fmt("Plain \x1B[2mFaint\x1B[22m Plain",         format!("Plain {ansi}Faint{ansi:#} Plain",     ansi=Effect::Faint));
    check_fmt("Plain \x1B[3mItalic\x1B[23m Plain",        format!("Plain {ansi}Italic{ansi:#} Plain",    ansi=Effect::Italic));
    check_fmt("Plain \x1B[4mUnderline\x1B[24m Plain",     format!("Plain {ansi}Underline{ansi:#} Plain", ansi=Effect::Underline));
    check_fmt("Plain \x1B[5mBlink\x1B[25m Plain",         format!("Plain {ansi}Blink{ansi:#} Plain",     ansi=Effect::Blink));
    check_fmt("Plain \x1B[7mReverse\x1B[27m Plain",       format!("Plain {ansi}Reverse{ansi:#} Plain",   ansi=Effect::Reverse));
    check_fmt("Plain \x1B[8mHidden\x1B[28m Plain",        format!("Plain {ansi}Hidden{ansi:#} Plain",    ansi=Effect::Hidden));
    check_fmt("Plain \x1B[9mStrike\x1B[29m Plain",        format!("Plain {ansi}Strike{ansi:#} Plain",    ansi=Effect::Strike));
    check_fmt(
        "Plain \x1B[1mBold \x1B[3mBold-Italic\x1B[23m Bold\x1B[22m Plain",
        format!("Plain {bold}Bold {italic}Bold-Italic{italic:#} Bold{bold:#} Plain",
            bold=Effect::Bold, italic=Effect::Italic
        )
    );
    check_fmt(
        "Plain \x1B[33mYellow \x1B[46mCyan\x1B[49m Yellow\x1B[39m Plain",
        format!("Plain {yellow}Yellow {cyan}Cyan{cyan:#} Yellow{yellow:#} Plain",
            yellow=Colour::Yellow, cyan=Colour::Cyan.bg()
        )
    );
}
