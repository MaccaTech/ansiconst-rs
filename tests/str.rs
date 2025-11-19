use ansiconst::ansi_code;

#[test]
fn test_str() {
    assert_eq!("\x1B[0m",   ansi_code!(Ansi::reset()    ));
    assert_eq!("\x1B[1m",   ansi_code!(Bold             ));
    assert_eq!("\x1B[2m",   ansi_code!(Faint            ));
    assert_eq!("\x1B[3m",   ansi_code!(Italic           ));
    assert_eq!("\x1B[4m",   ansi_code!(Underline        ));
    assert_eq!("\x1B[5m",   ansi_code!(Blink            ));
    assert_eq!("\x1B[7m",   ansi_code!(Reverse          ));
    assert_eq!("\x1B[8m",   ansi_code!(Hidden           ));
    assert_eq!("\x1B[9m",   ansi_code!(Strike           ));
    assert_eq!("\x1B[22m",  ansi_code!(Bold       .not()));
    assert_eq!("\x1B[22m",  ansi_code!(Faint      .not()));
    assert_eq!("\x1B[23m",  ansi_code!(Italic     .not()));
    assert_eq!("\x1B[24m",  ansi_code!(Underline  .not()));
    assert_eq!("\x1B[25m",  ansi_code!(Blink      .not()));
    assert_eq!("\x1B[27m",  ansi_code!(Reverse    .not()));
    assert_eq!("\x1B[28m",  ansi_code!(Hidden     .not()));
    assert_eq!("\x1B[29m",  ansi_code!(Strike     .not()));
    assert_eq!("\x1B[30m",  ansi_code!(Black            ));
    assert_eq!("\x1B[31m",  ansi_code!(Red              ));
    assert_eq!("\x1B[32m",  ansi_code!(Green            ));
    assert_eq!("\x1B[33m",  ansi_code!(Yellow           ));
    assert_eq!("\x1B[34m",  ansi_code!(Blue             ));
    assert_eq!("\x1B[35m",  ansi_code!(Purple           ));
    assert_eq!("\x1B[36m",  ansi_code!(Cyan             ));
    assert_eq!("\x1B[37m",  ansi_code!(White            ));
    #[cfg(feature="color256")]
    assert_eq!("\x1B[38;5;128m", ansi_code!(Color::num(128)));
    #[cfg(feature="rgb")]
    assert_eq!("\x1B[38;2;33;66;99m", ansi_code!(Color::rgb(33,66,99)));
    assert_eq!("\x1B[39m",  ansi_code!(Color::reset()   ));
    assert_eq!("\x1B[40m",  ansi_code!(Black       .bg()));
    assert_eq!("\x1B[41m",  ansi_code!(Red         .bg()));
    assert_eq!("\x1B[42m",  ansi_code!(Green       .bg()));
    assert_eq!("\x1B[43m",  ansi_code!(Yellow      .bg()));
    assert_eq!("\x1B[44m",  ansi_code!(Blue        .bg()));
    assert_eq!("\x1B[45m",  ansi_code!(Purple      .bg()));
    assert_eq!("\x1B[46m",  ansi_code!(Cyan        .bg()));
    assert_eq!("\x1B[47m",  ansi_code!(White       .bg()));
    #[cfg(feature="color256")]
    assert_eq!("\x1B[48;5;128m", ansi_code!(Color::num(128).bg()));
    #[cfg(feature="rgb")]
    assert_eq!("\x1B[48;2;33;66;99m", ansi_code!(Color::rgb(33,66,99).bg()));
    assert_eq!("\x1B[49m",  ansi_code!(Color::reset().bg()));
    assert_eq!("\x1B[90m",  ansi_code!(BrightBlack      ));
    assert_eq!("\x1B[91m",  ansi_code!(BrightRed        ));
    assert_eq!("\x1B[92m",  ansi_code!(BrightGreen      ));
    assert_eq!("\x1B[93m",  ansi_code!(BrightYellow     ));
    assert_eq!("\x1B[94m",  ansi_code!(BrightBlue       ));
    assert_eq!("\x1B[95m",  ansi_code!(BrightPurple     ));
    assert_eq!("\x1B[96m",  ansi_code!(BrightCyan       ));
    assert_eq!("\x1B[97m",  ansi_code!(BrightWhite      ));
    assert_eq!("\x1B[100m", ansi_code!(BrightBlack .bg()));
    assert_eq!("\x1B[101m", ansi_code!(BrightRed   .bg()));
    assert_eq!("\x1B[102m", ansi_code!(BrightGreen .bg()));
    assert_eq!("\x1B[103m", ansi_code!(BrightYellow.bg()));
    assert_eq!("\x1B[104m", ansi_code!(BrightBlue  .bg()));
    assert_eq!("\x1B[105m", ansi_code!(BrightPurple.bg()));
    assert_eq!("\x1B[106m", ansi_code!(BrightCyan  .bg()));
    assert_eq!("\x1B[107m", ansi_code!(BrightWhite .bg()));

    assert_eq!("\x1B[1;31m", ansi_code!(Red, Bold));
}
