use ansiconst::*;

#[test]
fn test_str() {
    assert_eq!("\x1B[0m",  ansi_code!(Ansi::reset()));
    assert_eq!("\x1B[30m", ansi_code!(Colour::Black));
    assert_eq!("\x1B[31m", ansi_code!(Colour::Red));
    assert_eq!("\x1B[32m", ansi_code!(Colour::Green));
    assert_eq!("\x1B[33m", ansi_code!(Colour::Yellow));
    #[cfg(feature="ansi256")]
    assert_eq!("\x1B[38;5;128m", ansi_code!(Colour::Ansi256(128)));
    #[cfg(feature="rgb")]
    assert_eq!("\x1B[38;2;33;66;99m", ansi_code!(Colour::Rgb(33,66,99)));

    assert_eq!("\x1B[40m", ansi_code!(Colour::Black.bg()));
    assert_eq!("\x1B[41m", ansi_code!(Colour::Red.bg()));
    assert_eq!("\x1B[42m", ansi_code!(Colour::Green.bg()));
    assert_eq!("\x1B[43m", ansi_code!(Colour::Yellow.bg()));
    #[cfg(feature="ansi256")]
    assert_eq!("\x1B[48;5;128m", ansi_code!(Colour::Ansi256(128).bg()));
    #[cfg(feature="rgb")]
    assert_eq!("\x1B[48;2;33;66;99m", ansi_code!(Colour::Rgb(33,66,99).bg()));
    assert_eq!("\x1B[1;31m", ansi_code!(Colour::Red, Effect::Bold));
}
