mod common;
mod build;
mod display;
use common::AnsiNode;
use build::{StyledStringBuild, StyledStringBuildPosition};
use display::StyledStringDisplay;
use crate::{Ansi, Styled};
use std::fmt;

/// A [`String`] containing [`Ansi`] styles, which can be overridden.
///
/// Instances of `StyledString`:
///
/// - are created using the [`styled_format!`](crate::styled_format!) macro,
/// or by calling [`Styled::to_styled_string`].
/// - can be converted to a plain [`String`] with [`to_string`](std::string::ToString::to_string),
/// or by using the [`format!`] macro.
///
/// *Note: this library provides a similarly-named [`Styled<T>`], which offers the same
/// functionality as `StyledString`, but stores an unformatted [`std::fmt::Display`] rather
/// than a formatted [`String`]. This may be less convenient for certain use cases,
/// but is more efficient since it avoids the runtime overhead of `StyledString`.*
///
/// ## Discussion
///
/// When [`Ansi`] styles are formatted into codes insides a [`String`], the references to
/// those styles are lost and they are "baked-into" the [`String`]'s contents.
/// This means one loses the ability to subsequently override those styles by nesting
/// that [`String`] inside a parent [`Styled<T>`]:
///
/// ### Example
/// ```
/// use ansiconst::styled_format_args;
///
/// let red: String = styled_format_args!(Red, "Red").to_string();
///
/// assert_eq!("\x1B[31mRed\x1B[39m", red);
///
/// let sadly_still_red = styled_format_args!(Ansi::no_ansi(), "{}", red).to_string();
///
/// assert_eq!("\x1B[31mRed\x1B[39m", sadly_still_red);
/// ```
///
/// `StyledString` solves this problem by keeping track of the contained [`Ansi`] styles
/// inside a formatted [`String`], such that those styles can subsequently be overridden:
///
/// ### Example
/// ```
/// use ansiconst::{styled_format, styled_format_args, StyledString};
///
/// let red: StyledString = styled_format!(Red, "Red");
///
/// assert_eq!("\x1B[31mRed\x1B[39m", red.to_string());
///
/// let no_longer_red = styled_format_args!(Ansi::no_ansi(), "{}", red).to_string();
///
/// assert_eq!("Red", no_longer_red);
/// ```
///
/// The flexibility afforded by `StyledString` comes at the expense of additional runtime overhead.
/// Therefore in performance critical situations, it is better not to create `StyledString`s,
/// but instead to use [`styled_format_args!`](crate::styled_format_args) to style output
/// at the moment when it is displayed.
///
/// Like [`Styled<T>`], `StyledString` uses [`thread_local!`] to pass style information between
/// nested styles and outer styles during formatting.
pub struct StyledString {
    template: String,
    ansi_nodes: Vec<AnsiNode>,
    max_depth: u8,
}

impl fmt::Display for StyledString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if ! StyledStringBuild::fmt_styled_string(f, self)? {
            StyledStringDisplay::fmt_styled_string(f, self)?;
        }
        Ok(())
    }
}

impl<T: fmt::Display> From<&Styled<T>> for StyledString {
    fn from(styled: &Styled<T>) -> Self {
        StyledStringBuild::from_styled(styled)
    }
}

pub(super) struct ToStyledString {
    build_position: StyledStringBuildPosition,
}

impl ToStyledString {
    #[inline]
    pub(super) fn fmt_styled_begin(f: &mut fmt::Formatter<'_>, ansi: Ansi) -> Result<Option<Self>, fmt::Error> {
        Ok(StyledStringBuild::fmt_styled_begin(f, ansi)?
            .map(|build_position| Self { build_position }))
    }

    #[inline]
    pub(super) fn fmt_styled_end(self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        StyledStringBuild::fmt_styled_end(f, self.build_position)
    }
}
