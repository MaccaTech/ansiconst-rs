//! Set default ANSI styles for writers and handles.
//!
//! It should be noted that the `Writer`s in this module are *not*
//! required to output ANSI styles. Printing styled output can be done with a
//! simple `println!`, for example:
//!
//! ```
//! use ansiconst::styled;
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
//!
//! *Note:* in order to configure the default ANSI style, trait [`AnsiWrite`] must be in scope.
//!
//! ### Examples
//!
//! ```
//! // This example assumes no relevant environment variables (FORCE_COLOR, NO_COLOR)
//! // have been set, and this is running on a terminal/tty.
//!
//! use ansiconst::{paintln, io::{ansiout, AnsiWrite}};
//!
//! // Prints "\x1B[35mPurple\x1B[39m\n"
//! paintln!(Purple, "Purple");
//!
//! // Manually set env variable and reset stdout's ANSI style to default
//! std::env::set_var("NO_COLOR", "1");
//! ansiout().auto_ansi();
//!
//! // Prints "Not Purple\n"
//! paintln!(Purple, "Not Purple");
//!
//! // Manually remove env variable and reset stdout's ANSI style to default
//! std::env::remove_var("NO_COLOR");
//! ansiout().auto_ansi();
//!
//! // Prints "\x1B[35mPurple\x1B[39m\n"
//! paintln!(Purple, "Purple");
//!
//! // Manually disable ANSI on stdout
//! ansiout().no_ansi();
//!
//! // Prints "Not Purple\n"
//! paintln!(Purple, "Not Purple");
//!
//! // Manually re-enable ANSI on stdout
//! ansiout().all_ansi();
//!
//! // Prints "\x1B[35mPurple\x1B[39m\n"
//! paintln!(Purple, "Purple");
//! ```

mod stream;
mod writer;

pub use stream::*;
pub use writer::*;

use std::{env, io};
use crate::Ansi;

/// Used to indicate if ANSI styles can/should be written by a `Writer`.
///
/// For example, ANSI codes should likely not be written to a non-terminal/tty.
/// For this reason, a blanket implementation of `AnsiPreference`
/// is provided for all implementors of [`IsTerminal`](io::IsTerminal)s, such
/// that the preferred ANSI style determined by this trait is, in the absence of
/// any relevant environment variables, based on the
/// [`is_terminal`](io::IsTerminal::is_terminal) method.
///
/// See examples in the [module-level documentation](crate::io).
pub trait AnsiPreference {
    /// Determines if this `Writer` prefers to enable ANSI styles in its output.
    ///
    /// E.g. if this `Writer` is a non-terminal/tty, the return value should be `false`.
    fn is_ansi_preferred(&self) -> bool;

    /// Determines if ANSI codes should be *enabled* because the`FORCE_COLOR`
    /// env variable has been set.
    fn is_ansi_forced(&self) -> bool {
        env::var_os("FORCE_COLOR").unwrap_or("".into()).len() > 0
    }

    /// Determines if ANSI codes should be *disabled* because the`NO_COLOR`
    /// env variable has been set.
    fn is_ansi_banned(&self) -> bool {
        env::var_os("NO_COLOR").unwrap_or("".into()).len() > 0
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
        if is_enabled { Ansi::empty() } else { Ansi::no_ansi() }
    }
}

impl<T: io::IsTerminal> AnsiPreference for T {
    fn is_ansi_preferred(&self) -> bool { self.is_terminal() }
}

/// A [`Write`](io::Write) that has a default [`Ansi`] style that may be configured.
///
/// The default style may be used to disable or override any ANSI styles nested in
/// the [`Arguments`](std::fmt::Arguments) passed to [`write_fmt()`](io::Write::write_fmt()).
///
/// Automatic configuration of the default [`Ansi`] style is done by calling
/// [`auto_ansi()`](AnsiWrite::auto_ansi).
///
/// See examples in the [module-level documentation](crate::io).
pub trait AnsiWrite: io::Write + AnsiPreference {
    /// Gets this `Writer`'s default [`Ansi`] style.
    ///
    /// See [`set_ansi`](AnsiWrite::set_ansi).
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
    /// use ansiconst::{ansi, paintln, io::{self, AnsiWrite}};
    ///
    /// io::ansiout().set_ansi(ansi!(Red.only()));
    /// paintln!(Blue, "Hello world");
    /// // Prints "\x1B[31mHello world\x1B[39m", i.e. red color (not blue)
    ///
    /// io::ansiout().set_ansi(ansi!(Ansi::no_ansi()));
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
    /// use ansiconst::{paintln, io::{self, AnsiWrite}};
    ///
    /// io::ansiout().all_ansi();
    /// paintln!(Red, "Hello world");
    /// // Prints "\x1B[31mHello world\x1B[39m", i.e. red color
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
        self.set_ansi(Ansi::empty())
    }

    /// Sets this `Writer`'s default [`Ansi`](AnsiWrite::set_ansi()) style to
    /// its [`preferred_ansi()`](AnsiPreference::preferred_ansi), such that
    /// rendering of nested ANSI styles during subsequent writes is enabled/disabled.
    fn auto_ansi(&mut self) {
        self.set_ansi(AnsiPreference::preferred_ansi(self))
    }
}
