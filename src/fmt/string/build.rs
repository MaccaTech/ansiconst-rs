use crate::{Ansi, Styled};
use super::{AnsiNode, StyledString};
use std::{cell::Cell, fmt};

pub(super) struct StyledStringBuildPosition {
    ansi_node_index: usize,
    parent_number_of_inner_ansis: u8,
}

pub(super) struct StyledStringBuild {
    ansi_nodes: Vec<AnsiNode>,
    number_of_inner_ansis: u8,
    max_depth: u8,
    depth: u8,
}

impl StyledStringBuild {
    thread_local! {
        static BUILD: Cell<Option<StyledStringBuild>> = Cell::new(None)
    }

    #[inline]
    fn new() -> Self {
        Self {
            ansi_nodes: Vec::new(),
            number_of_inner_ansis: 0,
            max_depth: 0,
            depth: 0,
        }
    }

    pub(super) fn from_styled<T: fmt::Display>(styled: &Styled<T>) -> StyledString {
        Self::BUILD.set(Some(Self::new()));
        let template = format!("{}", styled);
        let build = Self::BUILD.take().unwrap();

        StyledString {
            template,
            ansi_nodes: build.ansi_nodes,
            max_depth: build.max_depth,
        }
    }

    pub(super) fn fmt_styled_string(
        f: &mut fmt::Formatter<'_>,
        styled_string: &StyledString,
    ) -> Result<bool, fmt::Error> {
        Self::BUILD.with(|opt| {
            match opt.take() {
                None => Ok(false),
                Some(mut build) => {

                    if build.depth > 0 { f.write_str(AnsiNode::PLACEHOLDER)?; }
                    f.write_str(&styled_string.template)?;
                    if build.depth > 0 { f.write_str(AnsiNode::PLACEHOLDER)?; }

                    build.add_styled_string(styled_string);

                    opt.set(Some(build));
                    Ok(true)
                },
            }
        })
    }

    pub(super) fn fmt_styled_begin(
        f: &mut fmt::Formatter<'_>,
        ansi: Ansi,
    ) -> Result<Option<StyledStringBuildPosition>, fmt::Error> {
        Self::BUILD.with(|opt| {
            match opt.take() {
                None => Ok(None),
                Some(mut build) => {
                    if build.depth > 0 { f.write_str(AnsiNode::PLACEHOLDER)?; }

                    let position = build.styled_begin(ansi);

                    opt.set(Some(build));
                    Ok(Some(position))
                },
            }
        })
    }

    pub(super) fn fmt_styled_end(f: &mut fmt::Formatter<'_>, position: StyledStringBuildPosition) -> fmt::Result {
        Self::BUILD.with(|opt| {
            let mut build = opt.take().unwrap();

            build.styled_end(position);

            if build.depth > 0 { f.write_str(AnsiNode::PLACEHOLDER)?; }

            opt.set(Some(build));
            Ok(())
        })
    }

    #[inline]
    fn add_styled_string(&mut self, styled_string: &StyledString) {
        self.number_of_inner_ansis += 1;
        let max_depth = self.depth + styled_string.max_depth;
        if max_depth > self.max_depth { self.max_depth = max_depth; }
        self.ansi_nodes.extend_from_slice(&styled_string.ansi_nodes);
    }

    #[inline]
    fn styled_begin(&mut self, ansi: Ansi) -> StyledStringBuildPosition {
        let position = StyledStringBuildPosition {
            ansi_node_index:              self.ansi_nodes.len(),
            parent_number_of_inner_ansis: self.number_of_inner_ansis,
        };

        self.number_of_inner_ansis = 0;
        self.depth += 1;
        if self.depth > self.max_depth { self.max_depth += 1; }
        self.ansi_nodes.push(AnsiNode::new(ansi));

        position
    }

    #[inline]
    fn styled_end(&mut self, position: StyledStringBuildPosition) {
        let ansi_node = self.ansi_nodes.get_mut(position.ansi_node_index).unwrap();
        ansi_node.number_of_inner_ansis = self.number_of_inner_ansis;
        self.number_of_inner_ansis = position.parent_number_of_inner_ansis + 1;
        self.depth -= 1;
    }
}
