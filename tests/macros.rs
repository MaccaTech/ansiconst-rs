mod common;
use std::fmt::Write;

use ansiconst::{ansi, ansi_code, styled_format, styled_format_args};
use ansiconst::{styled_write, styled_writeln};
use ansiconst::{paint, paintln, epaint, epaintln};

#[test]
fn test_macros_trailing_comma() {
    assert_eq!(              ansi!( Red          ),
                             ansi!( Red,         ));
    assert_eq!(              ansi!( Red, Italic  ),
                             ansi!( Red, Italic, ));
    assert_eq!(         ansi_code!( Red          ),
                        ansi_code!( Red,         ));
    assert_eq!(         ansi_code!( Red, Italic  ),
                        ansi_code!( Red, Italic, ));
    assert_eq!(     styled_format!( Red, "Red"   ).to_string(),
                    styled_format!( Red, "Red",  ).to_string());
    assert_eq!(styled_format_args!( Red, "Red"   ).to_string(),
               styled_format_args!( Red, "Red",  ).to_string());

    let (mut buf1, mut buf2) = (String::new(), String::new());

    buf1.clear();
    buf2.clear();
    styled_write!(&mut buf1, Red, "Red" ).unwrap();
    styled_write!(&mut buf2, Red, "Red",).unwrap();
    assert_eq!(buf1, buf2);

    buf1.clear();
    buf2.clear();
    styled_writeln!(&mut buf1, Red, "Red" ).unwrap();
    styled_writeln!(&mut buf2, Red, "Red",).unwrap();
    assert_eq!(buf1, buf2);

    // Can't capture output, just try to use macro
    paint!   (Red, "Red",);
    paintln! (Red, "Red",);
    epaint!  (Red, "Red",);
    epaintln!(Red, "Red",);

}

#[test]
fn test_macros() {
    assert_eq_print!("Plain \x1B[31mRed\x1B[39m Plain",          format!("Plain {} Plain", styled_format_args!(Red, "Red")),
                                                                 format!("Plain {} Plain", styled_format!     (Red, "Red")));
    #[cfg(feature="color256")]
    assert_eq_print!("Plain \x1B[38;5;128mPurple\x1B[39m Plain", format!("Plain {} Plain", styled_format_args!(Color::num(128), "Purple")),
                                                                 format!("Plain {} Plain", styled_format!     (Color::num(128), "Purple")));
    assert_eq_print!("Plain \x1B[41mRed\x1B[49m Plain",          format!("Plain {} Plain", styled_format_args!(Red.bg(), "Red")),
                                                                 format!("Plain {} Plain", styled_format!     (Red.bg(), "Red")));
    #[cfg(feature="color256")]
    assert_eq_print!("Plain \x1B[48;5;128mPurple\x1B[49m Plain", format!("Plain {} Plain", styled_format_args!(Color::num(128).bg(), "Purple")),
                                                                 format!("Plain {} Plain", styled_format!     (Color::num(128).bg(), "Purple")));
    assert_eq_print!("Plain \x1B[1mBold\x1B[22m Plain",          format!("Plain {} Plain", styled_format_args!(Bold, "Bold")),
                                                                 format!("Plain {} Plain", styled_format!     (Bold, "Bold")));
    assert_eq_print!("Plain \x1B[2mFaint\x1B[22m Plain",         format!("Plain {} Plain", styled_format_args!(Faint, "Faint")),
                                                                 format!("Plain {} Plain", styled_format!     (Faint, "Faint")));
    assert_eq_print!("Plain \x1B[3mItalic\x1B[23m Plain",        format!("Plain {} Plain", styled_format_args!(Italic, "Italic")),
                                                                 format!("Plain {} Plain", styled_format!     (Italic, "Italic")));
    assert_eq_print!("Plain \x1B[4mUnderline\x1B[24m Plain",     format!("Plain {} Plain", styled_format_args!(Underline, "Underline")),
                                                                 format!("Plain {} Plain", styled_format!     (Underline, "Underline")));
    assert_eq_print!("Plain \x1B[5mBlink\x1B[25m Plain",         format!("Plain {} Plain", styled_format_args!(Blink, "Blink")),
                                                                 format!("Plain {} Plain", styled_format!     (Blink, "Blink")));
    assert_eq_print!("Plain \x1B[7mReverse\x1B[27m Plain",       format!("Plain {} Plain", styled_format_args!(Reverse, "Reverse")),
                                                                 format!("Plain {} Plain", styled_format!     (Reverse, "Reverse")));
    assert_eq_print!("Plain \x1B[8mHidden\x1B[28m Plain",        format!("Plain {} Plain", styled_format_args!(Hidden, "Hidden")),
                                                                 format!("Plain {} Plain", styled_format!     (Hidden, "Hidden")));
    assert_eq_print!("Plain \x1B[9mStrike\x1B[29m Plain",        format!("Plain {} Plain", styled_format_args!(Strike, "Strike")),
                                                                 format!("Plain {} Plain", styled_format!     (Strike, "Strike")));
    assert_eq_print!("Plain \x1B[1;2mBold Faint\x1B[22m Plain",  format!("Plain {} Plain", styled_format_args!(Bold, Faint, "Bold Faint")),
                                                                 format!("Plain {} Plain", styled_format!     (Bold, Faint, "Bold Faint")));

    assert_eq_print!(
        "Plain \x1B[1mBold-only \x1B[4mBoth Bold & Underline\x1B[24m Bold-only again\x1B[22m Plain",
        format!("Plain {} Plain",
            styled_format_args!(Bold, "Bold-only {} Bold-only again",
                styled_format_args!(Underline, "Both Bold & Underline")
            )
        ),
        format!("Plain {} Plain",
            styled_format!     (Bold, "Bold-only {} Bold-only again",
                styled_format_args!(Underline, "Both Bold & Underline")
            )
        ),
    );

    assert_eq_print!(
        "Plain \x1B[1mBold \x1B[22;3mItalic\x1B[23;1m Bold\x1B[22m Plain",
        format!("Plain {} Plain",
            styled_format_args!(Bold, "Bold {} Bold",
                styled_format_args!(Bold.not(), Italic, "Italic")
            )
        ),
        format!("Plain {} Plain",
            styled_format!     (Bold, "Bold {} Bold",
                styled_format_args!(Bold.not(), Italic, "Italic")
            )
        ),
    );

    assert_eq_print!(
        "Plain \x1B[33mYellow \x1B[31mRed\x1B[33m Yellow\x1B[39m Plain",
        format!("Plain {} Plain",
            styled_format_args!(Yellow, "Yellow {} Yellow",
                styled_format_args!(Red, "Red")
            )
        ),
        format!("Plain {} Plain",
            styled_format!     (Yellow, "Yellow {} Yellow",
                styled_format_args!(Red, "Red")
            )
        ),
    );

    assert_eq_print!(
        "Plain \x1B[1mBold \x1B[2mFaint\x1B[22;1m Bold\x1B[22m Plain",
        format!("Plain {} Plain",
            styled_format_args!(Bold, "Bold {} Bold",
                styled_format_args!(Faint, "Faint")
            )
        ),
        format!("Plain {} Plain",
            styled_format!     (Bold, "Bold {} Bold",
                styled_format_args!(Faint, "Faint")
            )
        ),
    );

    assert_eq_print!(
        "Plain \x1B[2mFaint \x1B[1mBold\x1B[22;2m Faint\x1B[22m Plain",
        format!("Plain {} Plain",
            styled_format_args!(Faint, "Faint {} Faint",
                styled_format_args!(Bold, "Bold")
            )
        ),
        format!("Plain {} Plain",
            styled_format!     (Faint, "Faint {} Faint",
                styled_format_args!(Bold, "Bold")
            )
        ),
    );

    assert_eq_print!(
        "Plain \x1B[1mBold \x1B[22;3mItalic\x1B[23;1m Bold\x1B[22m Plain",
        format!("Plain {} Plain",
            styled_format_args!(Bold, "Bold {} Bold",
                styled_format_args!(Italic.only(), "Italic"),
            )
        ),
        format!("Plain {} Plain",
            styled_format!     (Bold, "Bold {} Bold",
                styled_format_args!(Italic.only(), "Italic"),
            )
        ),
    );

    assert_eq_print!(
        "Plain \x1B[1mBold \x1B[22;2mFaint\x1B[22;1m Bold\x1B[22m Plain",
        format!("Plain {} Plain",
            styled_format_args!(Bold, "Bold {} Bold",
                styled_format_args!(Faint.only(), "Faint"),
            )
        ),
        format!("Plain {} Plain",
            styled_format!     (Bold, "Bold {} Bold",
                styled_format_args!(Faint.only(), "Faint"),
            )
        ),
    );

    assert_eq_print!(
        "Plain \x1B[1mBold \x1B[22;2mFaint \x1B[22;3mItalic \x1B[23;4mUnderline\x1B[24;3m Italic\x1B[23;2m Faint\x1B[22;1m Bold\x1B[22m Plain",
        format!("Plain {} Plain",
            styled_format_args!(Bold, "Bold {} Bold",
                styled_format_args!(Faint.only(), "Faint {} Faint",
                    styled_format_args!(Italic.only(), "Italic {} Italic",
                        styled_format_args!(Underline.only(), "Underline"),
                    )
                )
            )
        ),
        format!("Plain {} Plain",
            styled_format!     (Bold, "Bold {} Bold",
                styled_format_args!(Faint.only(), "Faint {} Faint",
                    styled_format_args!(Italic.only(), "Italic {} Italic",
                        styled_format!     (Underline.only(), "Underline"),
                    )
                )
            )
        ),
    );

    assert_eq_print!(
        "Plain \x1B[9mStrike \x1B[29;31mRed\x1B[39;9m \x1B[29;33mYellow\x1B[39;9m \x1B[29;32mGreen\x1B[39;9m \x1B[29;34mBlue\x1B[39;9m Strike\x1B[29m Plain",
        format!("Plain {} Plain",
            styled_format_args!(Strike, "Strike {} {} {} {} Strike",
                styled_format_args!(Red.only(),    "Red"),
                styled_format_args!(Yellow.only(), "Yellow"),
                styled_format_args!(Green.only(),  "Green"),
                styled_format_args!(Blue.only(),   "Blue"),
            )
        ),
        format!("Plain {} Plain",
            styled_format!     (Strike, "Strike {} {} {} {} Strike",
                styled_format_args!(Red.only(),    "Red"),
                styled_format_args!(Yellow.only(), "Yellow"),
                styled_format!     (Green.only(),  "Green"),
                styled_format_args!(Blue.only(),   "Blue"),
            )
        ),
    );
}
