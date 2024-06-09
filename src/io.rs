//! Set default ANSI styles for writers and handles.
//!
//! It should be noted that the `Writer`s in this module are *not*
//! required to output ANSI styles. Printing styled output can be done with a
//! simple `println!`, for example:
//!
//! ```
//! use ansiconst::{*, Colour::Red, Effect::Bold};
//!
//! // Note this uses standard println! that writes to std::io::Stdout
//! // (i.e. no ANSI-specific writer involved)
//! println!("{}", styled!(Red, Bold, "Hello world!"));
//! ```
//!
//! The purpose of this module is to provide support for:
//!
//! 1. Detecting the ANSI-styling capability of a `Writer` or `Stream` at runtime.
//! 2. Configuring a `Writer` or `Stream` to automatically disable/override nested ANSI
//! styles during writes.
//!
//! The above support is available as follows:
//!
//! - To set the default ANSI style for an existing `Writer`, wrap it in an [`AnsiWriter`].
//! - To set the default ANSI style when printing to `stdout`, `stderr`, use [`ansiout()`]
//! and [`ansierr()`].

mod stream;
mod writer;

pub use stream::*;
pub use writer::*;

use std::{env, io};
use crate::Ansi;

/// Used to indicate if ANSI styles can/should be written by a `Writer`.
///
/// E.g. ANSI codes should likely not be written to a non-terminal/tty.
pub trait AnsiPreference {
    /// Determines if this `Writer` prefers to enable ANSI styles in its output.
    ///
    /// E.g. if this `Writer` is a non-terminal/tty, the return value should be `false`.
    fn is_ansi_preferred(&self) -> bool;

    /// Determines if ANSI codes should be *enabled* because the`FORCE_COLOR`
    /// env variable has been set.
    fn is_ansi_forced(&self) -> bool {
        env::var_os("FORCE_COLOR").unwrap_or("".into()) == "1"
    }

    /// Determines if ANSI codes should be *disabled* because the`NO_COLOR`
    /// env variable has been set.
    fn is_ansi_banned(&self) -> bool {
        env::var_os("NO_COLOR").is_some()
    }

    /// Creates an [`Ansi`] intended to be used to enable/disable ANSI styles
    /// in a `Writer`.
    ///
    /// In order to determine whether or not to return the enabling-type or
    /// disabling-type [`Ansi`] instance, this method calls the other `is_ansi_*()`
    /// methods in this trait.
    fn preferred_ansi(&self) -> Ansi {
        let is_enabled = if self.is_ansi_forced() {
            true
        } else if self.is_ansi_banned() {
            false
        } else {
            self.is_ansi_preferred()
        };
        if is_enabled { Ansi::unspecified() } else { Ansi::no_ansi() }
    }
}

/// A [`Write`](io::Write) that has a default [`Ansi`] style that may be configured.
///
/// The default style may be used to disable or override any ANSI styles nested in
/// the [`Arguments`](std::fmt::Arguments) passed to [`write_fmt()`](io::Write::write_fmt()).
///
/// Automatic configuration of the default [`Ansi`] style is also supported -
/// see [`auto_ansi()`](AnsiWrite::auto_ansi).
pub trait AnsiWrite: io::Write + AnsiPreference {
    /// Gets this `Writer`'s default [`Ansi`] style.
    ///
    /// The default style may be used to disable or override any ANSI styles nested in
    /// the [`Arguments`](std::fmt::Arguments) passed to [`write_fmt()`](io::Write::write_fmt()).
    ///
    /// E.g. if set to [`Ansi::no_ansi()`], then any such nested ANSI styles will be suppressed.
    fn ansi(&self) -> Ansi;

    /// Sets this `Writer`'s default [`Ansi`] style.
    ///
    /// The default style may be used to disable or override any ANSI styles nested in
    /// the [`Arguments`](std::fmt::Arguments) passed to [`write_fmt()`](io::Write::write_fmt()).
    ///
    /// E.g. if set to [`Ansi::no_ansi()`], then any such nested ANSI styles will be suppressed.
    ///
    /// ### Examples
    ///
    /// ```
    /// use ansiconst::{*, io::*, Colour::*};
    ///
    /// io::ansiout().set_ansi(Red.only());
    /// paintln!(Blue, "Hello world");
    /// // Prints "\x1B[31mHello world\x1B[39m", i.e. red colour (not blue)
    ///
    /// io::ansiout().set_ansi(Ansi::no_ansi());
    /// paintln!(Red, "Hello world");
    /// // Prints "Hello world", i.e. without any ANSI codes
    ///
    /// ```
    fn set_ansi(&mut self, ansi: Ansi);

    /// Determines whether this `Writer`'s default [`Ansi`](AnsiWrite::ansi()) style prohibits
    /// writing of all nested ANSI styles.
    fn is_no_ansi(&self) -> bool {
        self.ansi().is_no_ansi()
    }

    /// Sets this `Writer`'s default [`Ansi`](AnsiWrite::set_ansi()) style such that
    /// rendering of nested ANSI styles during subsequent writes is disabled.
    ///
    /// ### Examples
    ///
    /// ```
    /// use ansiconst::{*, io::*, Colour::*};
    ///
    /// io::ansiout().all_ansi();
    /// paintln!(Red, "Hello world");
    /// // Prints "\x1B[31mHello world\x1B[39m", i.e. red colour
    ///
    /// io::ansiout().no_ansi();
    /// paintln!(Red, "Hello world");
    /// // Prints "Hello world", i.e. without any ANSI codes
    ///
    /// ```
    fn no_ansi(&mut self) {
        self.set_ansi(Ansi::no_ansi())
    }

    /// Determines whether this `Writer`'s default [`Ansi`](AnsiWrite::ansi()) style allows
    /// writing of all nested ANSI styles.
    fn is_all_ansi(&self) -> bool {
        self.ansi().is_empty()
    }

    /// Sets this `Writer`'s default [`Ansi`](AnsiWrite::set_ansi()) style such that
    /// rendering of nested ANSI styles during subsequent writes is enabled.
    ///
    /// See [`no_ansi()`](AnsiWrite::no_ansi) for examples.
    fn all_ansi(&mut self) {
        self.set_ansi(Ansi::unspecified())
    }

    /// Sets this `Writer`'s default [`Ansi`](AnsiWrite::set_ansi()) style to
    /// its [`preferred_ansi()`](AnsiPreference::preferred_ansi), such that
    /// rendering of nested ANSI styles during subsequent writes is enabled/disabled.
    fn auto_ansi(&mut self) {
        self.set_ansi(AnsiPreference::preferred_ansi(self))
    }
}
