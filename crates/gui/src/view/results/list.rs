//! How the rows are drawn in the list: formatting codes, sizes and wrapping.

use fltk::draw;
use fltk::enums::Color;
use rustshot_core::UnreadableIn;

use super::rows::Row;
use crate::theme;
use crate::types::Filter;


// Formatting codes of the list start with this character instead of '@',
// because registry data can contain '@'.
pub(super) const FORMAT: char = '\u{1}';

// Font sizes of the key path and of the value.
const KEY_SIZE: i32 = 12;
const VALUE_SIZE: i32 = 13;

// Height of the empty lines used as space: around each change, between the
// key and the value, and the uncoloured gap between two changes.
const AROUND: i32 = 10;
const BETWEEN: i32 = 4;
const GAP: i32 = 3;


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
// key, the value below, more space and a small gap without colour before the
// next change. Long texts go on several lines.
pub(super) fn lines(row: &Row, fit: &Fit) -> Vec<String> {
    // All is the filter of no row: it only exists for the buttons. A key that
    // became unreadable in the 2nd shot stands out: it may have been locked.
    let (symbol, color, background) = match (row.filter, row.unreadable_in) {
        (Filter::KeysAdded | Filter::ValuesAdded, _) => ("+", theme::SUCCESS, theme::ADDED_BG),
        (Filter::KeysDeleted | Filter::ValuesDeleted, _) => ("-", theme::DANGER, theme::DELETED_BG),
        (Filter::Modified | Filter::All, _) => ("~", theme::WARNING, theme::MODIFIED_BG),
        (Filter::NotCompared, Some(UnreadableIn::After)) => ("!", theme::WARNING, theme::CARD),
        (Filter::NotCompared, _) => ("?", theme::TEXT_MUTED, theme::CARD),
    };

    // Each column of each line is filled, so the change is one coloured block.
    let fill = fill(background);
    let mut lines = vec![format!("{fill}{}", space(AROUND))];

    for (i, part) in wrap(&row.key, fit.key).iter().enumerate() {
        let symbol = if i == 0 {
            format!("{FORMAT}C{}{FORMAT}b{FORMAT}S14{symbol}", color.bits())
        } else {
            String::new()
        };

        let key = style(theme::TEXT_SECONDARY, KEY_SIZE);
        lines.push(format!("{fill}{symbol}\t{fill}{key}{part}"));
    }

    lines.push(format!("{fill}{}", space(BETWEEN)));

    for part in wrap(&row.text, fit.value) {
        let value = style(theme::TEXT, VALUE_SIZE);
        lines.push(format!("{fill}\t{fill}{value}{part}"));
    }

    lines.push(format!("{fill}{}", space(AROUND)));
    lines.push(space(GAP));
    lines
}

// Line shown when no change passes the filter and the search.
pub(super) fn empty() -> String {
    format!("\t{}(no changes match)", style(theme::TEXT_MUTED, VALUE_SIZE))
}


// Background of a column. The list does not paint it on the selected line.
fn fill(color: Color) -> String {
    format!("{FORMAT}B{}", color.bits())
}

// Color, fixed-width font and size, at the start of a column.
fn style(color: Color, size: i32) -> String {
    format!("{FORMAT}C{}{FORMAT}f{FORMAT}S{size}", color.bits())
}

// An empty line of the given font size, used as vertical space.
fn space(size: i32) -> String {
    format!("{FORMAT}S{size}{FORMAT}.")
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


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn short_text_is_one_line() {
        assert_eq!(wrap(r"HKLM\Software", 40), [r"HKLM\Software"]);
    }

    #[test]
    fn long_text_is_cut_after_a_backslash() {
        let lines = wrap(r"HKLM\SOFTWARE\Microsoft\Windows", 20);

        assert_eq!(lines, [r"HKLM\SOFTWARE\", r"Microsoft\Windows"]);
    }

    #[test]
    fn text_without_places_to_cut_is_cut_at_the_limit() {
        let lines = wrap(&"a".repeat(25), 10);

        assert_eq!(lines, ["a".repeat(10), "a".repeat(10), "a".repeat(5)]);
    }

    #[test]
    fn wrapping_keeps_every_character() {
        let text = r"HKU\S-1-5-21-1004\Software\Classes\CLSID\{0000-1111}, more text";

        for max in [10, 17, 33] {
            let lines = wrap(text, max);

            assert_eq!(lines.concat(), text);
            assert!(lines.iter().all(|line| line.chars().count() <= max));
        }
    }
}
