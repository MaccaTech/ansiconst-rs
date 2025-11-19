use crate::Ansi;

#[derive(PartialEq, Eq, Clone, Copy)]
pub(super) struct AnsiNode {
    pub(super) ansi: Ansi,
    pub(super) number_of_inner_ansis: u8,
}

impl AnsiNode {
    pub(super) const PLACEHOLDER: &'static str = "\x1B[?m";

    #[inline]
    pub(super) fn new(ansi: Ansi) -> Self { Self { ansi, number_of_inner_ansis: 0 } }
}
