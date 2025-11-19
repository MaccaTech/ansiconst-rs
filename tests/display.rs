mod common;

use ansiconst::*;

#[test]
fn test_display_codes() {
    assert_eq!(format!("{}", Ansi::reset()            ), "\x1B[0m"  );
    assert_eq!(format!("{}", Effect::Bold             ), "\x1B[1m"  );
    assert_eq!(format!("{}", Effect::Faint            ), "\x1B[2m"  );
    assert_eq!(format!("{}", Effect::Italic           ), "\x1B[3m"  );
    assert_eq!(format!("{}", Effect::Underline        ), "\x1B[4m"  );
    assert_eq!(format!("{}", Effect::Blink            ), "\x1B[5m"  );
    assert_eq!(format!("{}", Effect::Reverse          ), "\x1B[7m"  );
    assert_eq!(format!("{}", Effect::Hidden           ), "\x1B[8m"  );
    assert_eq!(format!("{}", Effect::Strike           ), "\x1B[9m"  );
    assert_eq!(format!("{}", Effect::Bold       .not()), "\x1B[22m" );
    assert_eq!(format!("{}", Effect::Faint      .not()), "\x1B[22m" );
    assert_eq!(format!("{}", Effect::Italic     .not()), "\x1B[23m" );
    assert_eq!(format!("{}", Effect::Underline  .not()), "\x1B[24m" );
    assert_eq!(format!("{}", Effect::Blink      .not()), "\x1B[25m" );
    assert_eq!(format!("{}", Effect::Reverse    .not()), "\x1B[27m" );
    assert_eq!(format!("{}", Effect::Hidden     .not()), "\x1B[28m" );
    assert_eq!(format!("{}", Effect::Strike     .not()), "\x1B[29m" );
    assert_eq!(format!("{}", Color::Black             ), "\x1B[30m" );
    assert_eq!(format!("{}", Color::Red               ), "\x1B[31m" );
    assert_eq!(format!("{}", Color::Green             ), "\x1B[32m" );
    assert_eq!(format!("{}", Color::Yellow            ), "\x1B[33m" );
    assert_eq!(format!("{}", Color::Blue              ), "\x1B[34m" );
    assert_eq!(format!("{}", Color::Purple            ), "\x1B[35m" );
    assert_eq!(format!("{}", Color::Cyan              ), "\x1B[36m" );
    assert_eq!(format!("{}", Color::White             ), "\x1B[37m" );
    #[cfg(feature="rgb")]
    assert_eq!(format!("{}", Color::rgb(45,67,89)     ), "\x1B[38;2;45;67;89m");
    #[cfg(feature="color256")]
    assert_eq!(format!("{}", Color::num(255)          ), "\x1B[38;5;255m");
    assert_eq!(format!("{}", Color::reset()           ), "\x1B[39m" );
    assert_eq!(format!("{}", Color::Black        .bg()), "\x1B[40m" );
    assert_eq!(format!("{}", Color::Red          .bg()), "\x1B[41m" );
    assert_eq!(format!("{}", Color::Green        .bg()), "\x1B[42m" );
    assert_eq!(format!("{}", Color::Yellow       .bg()), "\x1B[43m" );
    assert_eq!(format!("{}", Color::Blue         .bg()), "\x1B[44m" );
    assert_eq!(format!("{}", Color::Purple       .bg()), "\x1B[45m" );
    assert_eq!(format!("{}", Color::Cyan         .bg()), "\x1B[46m" );
    assert_eq!(format!("{}", Color::White        .bg()), "\x1B[47m" );
    #[cfg(feature="rgb")]
    assert_eq!(format!("{}", Color::rgb(45,67,89).bg()), "\x1B[48;2;45;67;89m");
    #[cfg(feature="color256")]
    assert_eq!(format!("{}", Color::num(255)     .bg()), "\x1B[48;5;255m");
    assert_eq!(format!("{}", Color::reset()      .bg()), "\x1B[49m" );
    assert_eq!(format!("{}", Color::BrightBlack       ), "\x1B[90m" );
    assert_eq!(format!("{}", Color::BrightRed         ), "\x1B[91m" );
    assert_eq!(format!("{}", Color::BrightGreen       ), "\x1B[92m" );
    assert_eq!(format!("{}", Color::BrightYellow      ), "\x1B[93m" );
    assert_eq!(format!("{}", Color::BrightBlue        ), "\x1B[94m" );
    assert_eq!(format!("{}", Color::BrightPurple      ), "\x1B[95m" );
    assert_eq!(format!("{}", Color::BrightCyan        ), "\x1B[96m" );
    assert_eq!(format!("{}", Color::BrightWhite       ), "\x1B[97m" );
    assert_eq!(format!("{}", Color::BrightBlack  .bg()), "\x1B[100m");
    assert_eq!(format!("{}", Color::BrightRed    .bg()), "\x1B[101m");
    assert_eq!(format!("{}", Color::BrightGreen  .bg()), "\x1B[102m");
    assert_eq!(format!("{}", Color::BrightYellow .bg()), "\x1B[103m");
    assert_eq!(format!("{}", Color::BrightBlue   .bg()), "\x1B[104m");
    assert_eq!(format!("{}", Color::BrightPurple .bg()), "\x1B[105m");
    assert_eq!(format!("{}", Color::BrightCyan   .bg()), "\x1B[106m");
    assert_eq!(format!("{}", Color::BrightWhite  .bg()), "\x1B[107m");
}

#[test]
fn test_display() {
    assert_eq_print!("Plain \x1B[31mRed\x1B[39m Plain",          format!("Plain {ansi}Red{ansi:#} Plain",       ansi=Color::Red));
    #[cfg(feature="color256")]
    assert_eq_print!("Plain \x1B[38;5;128mPurple\x1B[39m Plain", format!("Plain {ansi}Purple{ansi:#} Plain",    ansi=Color::num(128)));
    assert_eq_print!("Plain \x1B[41mRed\x1B[49m Plain",          format!("Plain {ansi}Red{ansi:#} Plain",       ansi=Color::Red.bg()));
    #[cfg(feature="color256")]
    assert_eq_print!("Plain \x1B[48;5;128mPurple\x1B[49m Plain", format!("Plain {ansi}Purple{ansi:#} Plain",    ansi=Color::num(128).bg()));
    assert_eq_print!("Plain \x1B[1mBold\x1B[22m Plain",          format!("Plain {ansi}Bold{ansi:#} Plain",      ansi=Effect::Bold));
    assert_eq_print!("Plain \x1B[2mFaint\x1B[22m Plain",         format!("Plain {ansi}Faint{ansi:#} Plain",     ansi=Effect::Faint));
    assert_eq_print!("Plain \x1B[3mItalic\x1B[23m Plain",        format!("Plain {ansi}Italic{ansi:#} Plain",    ansi=Effect::Italic));
    assert_eq_print!("Plain \x1B[4mUnderline\x1B[24m Plain",     format!("Plain {ansi}Underline{ansi:#} Plain", ansi=Effect::Underline));
    assert_eq_print!("Plain \x1B[5mBlink\x1B[25m Plain",         format!("Plain {ansi}Blink{ansi:#} Plain",     ansi=Effect::Blink));
    assert_eq_print!("Plain \x1B[7mReverse\x1B[27m Plain",       format!("Plain {ansi}Reverse{ansi:#} Plain",   ansi=Effect::Reverse));
    assert_eq_print!("Plain \x1B[8mHidden\x1B[28m Plain",        format!("Plain {ansi}Hidden{ansi:#} Plain",    ansi=Effect::Hidden));
    assert_eq_print!("Plain \x1B[9mStrike\x1B[29m Plain",        format!("Plain {ansi}Strike{ansi:#} Plain",    ansi=Effect::Strike));
    assert_eq_print!(
        "Plain \x1B[1mBold \x1B[3mBold-Italic\x1B[23m Bold\x1B[22m Plain",
        format!("Plain {bold}Bold {italic}Bold-Italic{italic:#} Bold{bold:#} Plain",
            bold=Effect::Bold, italic=Effect::Italic
        )
    );
    assert_eq_print!(
        "Plain \x1B[33mYellow \x1B[46mCyan\x1B[49m Yellow\x1B[39m Plain",
        format!("Plain {yellow}Yellow {cyan}Cyan{cyan:#} Yellow{yellow:#} Plain",
            yellow=Color::Yellow, cyan=Color::Cyan.bg()
        )
    );
}
