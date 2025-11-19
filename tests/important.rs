mod common;

use ansiconst::{styled_format, styled_format_args};

#[test]
fn test_important() {
    assert_eq_print!(
        "\x1B[34mBlue \x1B[39;3mitalic\x1B[23;34m blue\x1B[39m",
        styled_format_args!(Blue, "Blue {} blue",
            styled_format_args!(Italic.only(), "italic")
        ).to_string()
    );
    assert_eq_print!(
        "no ansi",
        styled_format_args!(Ansi::no_ansi(), "{}",
            styled_format_args!(Bold, Red, "no ansi")
        ).to_string()
    );
    assert_eq_print!(
        "\x1B[34mBlue still blue\x1B[39m",
        styled_format_args!(Blue.important(), "Blue {}",
            styled_format_args!(Red, "still blue")
        ).to_string()
    );
    assert_eq_print!(
        "\x1B[34mBlue \x1B[31mred\x1B[34m\x1B[39m",
        styled_format_args!(Blue.important(), "Blue {}",
            styled_format_args!(Red.important(), "red")
        ).to_string()
    );
    assert_eq_print!(
        "\x1B[0;1mBold-only still bold-only\x1B[22m",
        styled_format_args!(Bold.only().important(), "Bold-only {}",
            styled_format_args!(Italic, Red, "still bold-only")
        ).to_string()
    );
    assert_eq_print!(
        "\x1B[0;1mBold-only \x1B[22;2mfaint-only\x1B[22;1m\x1B[22m",
        styled_format_args!(Bold.only().important(), "Bold-only {}",
            styled_format_args!(Faint.only().important(), "faint-only")
        ).to_string()
    );

    assert_eq_print!(
        "Plain \x1B[31mRed \x1B[33mYellow \x1B[32mGreen \x1B[34mBlue\x1B[32m\x1B[33m\x1B[31m\x1B[39m Plain",
        format!("Plain {} Plain",
            styled_format_args!(Red, "Red {}",
                styled_format_args!(Yellow, "Yellow {}",
                    styled_format_args!(Green, "Green {}",
                        styled_format_args!(Blue, "Blue"),
                    )
                )
            )
        ),
        format!("Plain {} Plain",
            styled_format!     (Red, "Red {}",
                styled_format!     (Yellow, "Yellow {}",
                    styled_format!     (Green, "Green {}",
                        styled_format!     (Blue, "Blue"),
                    )
                )
            )
        ),
    );

    assert_eq_print!(
        "Plain \x1B[31mRed Yellow Green Blue\x1B[39m Plain",
        format!("Plain {} Plain",
            styled_format_args!(Red.important(), "Red {}",
                styled_format_args!(Yellow, "Yellow {}",
                    styled_format_args!(Green, "Green {}",
                        styled_format_args!(Blue, "Blue"),
                    )
                )
            )
        ),
        format!("Plain {} Plain",
            styled_format!     (Red.important(), "Red {}",
                styled_format!     (Yellow, "Yellow {}",
                    styled_format!     (Green, "Green {}",
                        styled_format!     (Blue, "Blue"),
                    )
                )
            )
        ),
        format!("Plain {} Plain",
            styled_format!     (Red.important(), "Red {}",
                styled_format_args!(Yellow, "Yellow {}",
                    styled_format!     (Green, "Green {}",
                        styled_format_args!(Blue, "Blue"),
                    )
                )
            )
        ),
        format!("Plain {} Plain",
            styled_format_args!(Red.important(), "Red {}",
                styled_format!    (Yellow, "Yellow {}",
                    styled_format_args!(Green, "Green {}",
                        styled_format!     (Blue, "Blue"),
                    )
                )
            )
        ),
    );
}
