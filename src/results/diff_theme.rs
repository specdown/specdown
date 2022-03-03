use crossterm::style::Stylize;
use std::borrow::Cow;
use termdiff::{ArrowsColorTheme, Theme};

#[derive(Debug)]
pub struct DiffTheme {}
const BASE_THEME: ArrowsColorTheme = ArrowsColorTheme {};
pub const DIFF_THEME: DiffTheme = DiffTheme {};

impl Theme for DiffTheme {
    fn highlight_insert<'this>(&self, input: &'this str) -> Cow<'this, str> {
        BASE_THEME.highlight_insert(input)
    }

    fn highlight_delete<'this>(&self, input: &'this str) -> Cow<'this, str> {
        BASE_THEME.highlight_delete(input)
    }

    fn equal_content<'this>(&self, input: &'this str) -> Cow<'this, str> {
        BASE_THEME.equal_content(input)
    }

    fn delete_content<'this>(&self, input: &'this str) -> Cow<'this, str> {
        BASE_THEME.delete_content(input)
    }

    fn equal_prefix<'this>(&self) -> Cow<'this, str> {
        BASE_THEME.equal_prefix()
    }

    fn delete_prefix<'this>(&self) -> Cow<'this, str> {
        BASE_THEME.delete_prefix()
    }

    fn insert_line<'this>(&self, input: &'this str) -> Cow<'this, str> {
        BASE_THEME.insert_line(input)
    }

    fn insert_prefix<'this>(&self) -> Cow<'this, str> {
        BASE_THEME.insert_prefix()
    }

    fn line_end<'this>(&self) -> Cow<'this, str> {
        BASE_THEME.line_end()
    }

    fn header<'this>(&self) -> Cow<'this, str> {
        format!("{} / {}\n", "< expected".red(), "> actual".green()).into()
    }
}
