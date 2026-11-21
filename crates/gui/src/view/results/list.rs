//! How the rows are drawn in the list: formatting codes, sizes and wrapping.

use fltk::draw;
use fltk::enums::Color;

use super::rows::Row;
use crate::theme;
use crate::types::Filter;


// Formatting codes of the list start with this character instead of '@',
// because registry data can contain '@'.
pub(super) const FORMAT: char = '\u{1}';

// Font sizes of the key path and of the value.
const KEY_SIZE: i32 = 12;
const VALUE_SIZE: i32 = 13;

// Height of the empty lines used as space: around each change, and between
// the key and the value.
const AROUND: i32 = 10;
const BETWEEN: i32 = 4;


// How many characters of each font size fit in the list.
pub(super) struct Fit {
    key: usize,
    value: usize,
}


impl Fit {
    // The fonts are fixed width: every character is as wide as "M".
    pub(super) fn new(width: i32) -> Self {
        Fit {
            key: chars_that_fit(width, KEY_SIZE),
            value: chars_that_fit(width, VALUE_SIZE),
        }
    }
}


// Lines of the list for a change: some space, the coloured symbol and the
// key, the value below, more space and a separator. Long texts go on
// several lines.
pub(super) fn lines(row: &Row, fit: &Fit) -> Vec<String> {
    // All is the filter of no row: it only exists for the buttons.
    let (symbol, color) = match row.filter {
        Filter::KeysAdded | Filter::ValuesAdded => ("+", theme::SUCCESS),
        Filter::KeysDeleted | Filter::ValuesDeleted => ("-", theme::DANGER),
        Filter::Modified | Filter::All => ("~", theme::WARNING),
    };

    let mut lines = vec![space(AROUND)];

    for (i, part) in wrap(&row.key, fit.key).iter().enumerate() {
        let symbol = if i == 0 {
            format!("{FORMAT}C{}{FORMAT}b{FORMAT}S14{symbol}", color.bits())
        } else {
            String::new()
        };

        lines.push(format!("{symbol}\t{}{part}", style(theme::TEXT_SECONDARY, KEY_SIZE)));
    }

    lines.push(space(BETWEEN));

    for part in wrap(&row.text, fit.value) {
        lines.push(format!("\t{}{part}", style(theme::TEXT, VALUE_SIZE)));
    }

    lines.push(space(AROUND));
    lines.push(separator());
    lines
}


// Line shown when no change passes the filter and the search.
pub(super) fn empty() -> String {
    format!("\t{}(no changes match)", style(theme::TEXT_MUTED, VALUE_SIZE))
}


// Color, fixed-width font and size, at the start of a column.
fn style(color: Color, size: i32) -> String {
    format!("{FORMAT}C{}{FORMAT}f{FORMAT}S{size}", color.bits())
}


// An empty line of the given font size, used as vertical space.
fn space(size: i32) -> String {
    format!("{FORMAT}S{size}{FORMAT}.")
}


// A horizontal line between two changes.
fn separator() -> String {
    format!("{FORMAT}S8{FORMAT}-")
}


fn chars_that_fit(width: i32, size: i32) -> usize {
    draw::set_font(theme::FONT_MONO, size);
    let char_width = draw::width("M").max(1.0);

    ((f64::from(width) / char_width) as usize).max(10)
}


// Splits `text` in lines of at most `max` characters, cutting after a '\',
// a space or a comma when there is one in the second half of the line.
fn wrap(text: &str, max: usize) -> Vec<String> {
    let chars: Vec<char> = text.chars().collect();
    let mut lines = Vec::new();
    let mut start = 0;

    while chars.len() - start > max {
        let end = start + max;
        let cut = chars[start..end]
            .iter()
            .rposition(|c| matches!(c, '\\' | ' ' | ','))
            .map(|i| start + i + 1)
            .filter(|cut| *cut > start + max / 2)
            .unwrap_or(end);

        lines.push(chars[start..cut].iter().collect());
        start = cut;
    }

    lines.push(chars[start..].iter().collect());
    lines
}
