use super::{AnsiNode, StyledString};
use super::super::StyledDisplay;
use std::fmt;

pub(super) struct StyledStringDisplay<'a> {
    remainder: &'a str,
    stack: Vec<AnsiNode>,
}

impl StyledStringDisplay<'_> {
    pub(super) fn fmt_styled_string(f: &mut fmt::Formatter<'_>, styled_string: &StyledString) -> fmt::Result {
        let remainder = styled_string.template.as_str();
        let mut stack: Vec<AnsiNode> = Vec::with_capacity(styled_string.max_depth as usize + 1);
        stack.push(AnsiNode { ansi: StyledDisplay::ansi(), number_of_inner_ansis: 1 });

        let mut display = StyledStringDisplay { remainder, stack };
        for ansi_node in &styled_string.ansi_nodes {
            display.fmt_node(f, ansi_node)?;
        }
        Ok(())
    }

    fn fmt_node(&mut self, f: &mut fmt::Formatter<'_>, ansi_node: &AnsiNode) -> fmt::Result {
        self.fmt_ansi_begin(f, ansi_node)?;
        self.fmt_content(f)?;
        while self.is_ansi_end() {
            self.fmt_ansi_end(f)?;
            self.fmt_content(f)?;
        }
        Ok(())
    }

    fn fmt_ansi_begin(&mut self, f: &mut fmt::Formatter<'_>, ansi_node: &AnsiNode) -> fmt::Result {
        let old_ansi_node = self.stack.last_mut().unwrap();
        old_ansi_node.number_of_inner_ansis -= 1;
        let old_ansi = old_ansi_node.ansi;
        let new_ansi = old_ansi.then(ansi_node.ansi);
        let old_to_new = old_ansi.transition(new_ansi);
        self.stack.push(AnsiNode { ansi: new_ansi.only(), number_of_inner_ansis: ansi_node.number_of_inner_ansis });
        old_to_new.fmt_no_alternate(f)
    }

    fn fmt_content(&mut self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.remainder.len() > 0 {
            if let Some((prefix, remainder)) = self.remainder.split_once(AnsiNode::PLACEHOLDER) {
                self.remainder = remainder;
                f.write_str(prefix)?;
            } else {
                f.write_str(self.remainder)?;
                self.remainder = "";
            }
        }
        Ok(())
    }

    #[inline]
    fn is_ansi_end(&self) -> bool {
        self.stack.len() > 1
        && self.stack.last().unwrap().number_of_inner_ansis == 0
    }

    fn fmt_ansi_end(&mut self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let new_ansi = self.stack.pop().unwrap().ansi;
        let old_ansi = self.stack.last().unwrap().ansi;
        let new_to_old = new_ansi.transition(old_ansi);
        new_to_old.fmt_no_alternate(f)
    }
}
