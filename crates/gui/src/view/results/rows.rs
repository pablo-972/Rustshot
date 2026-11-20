use fltk::draw;
use fltk::enums::Color;
use kernel::snapshot::{RegistryValue, SnapshotDiff};

use crate::theme;
use crate::types::Filter;


// Bytes shown of a binary value before cutting it.
const BINARY_PREVIEW: usize = 32;

// Formatting codes of the list start with this character instead of '@',
// because registry data can contain '@'.
pub(super) const FORMAT: char = '\u{1}';

// Font sizes of the key path and of the value.
const KEY_SIZE: i32 = 11;
const VALUE_SIZE: i32 = 12;


#[derive(Clone, Copy)]
enum Kind {
    Added,
    Deleted,
    Modified,
}


// One change of the diff, ready to be shown.
pub(super) struct Row {
    pub filter: Filter,
    kind: Kind,
    key: String,
    text: String,
    search: String,
}


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


impl Row {
    fn new(
        filter: Filter,
        kind: Kind,
        key: &str,
        text: String
    ) -> Self {
        let key = clean(key);
        let text = clean(&text);

        Row {
            filter,
            kind,
            search: format!("{key} {text}").to_lowercase(),
            key,
            text,
        }
    }

    // `query` must be lowercase already.
    pub(super) fn matches(&self, filter: Filter, query: &str) -> bool {
        let kind = filter == Filter::All || filter == self.filter;
        kind && (query.is_empty() || self.search.contains(query))
    }

    // Lines of the list for this change: the coloured symbol and the key,
    // the value below, and a small gap. Long texts go on several lines.
    pub(super) fn lines(&self, fit: &Fit) -> Vec<String> {
        let (symbol, color) = match self.kind {
            Kind::Added => ("+", theme::SUCCESS),
            Kind::Deleted => ("-", theme::DANGER),
            Kind::Modified => ("~", theme::WARNING),
        };

        let mut lines = Vec::new();

        for (i, part) in wrap(&self.key, fit.key).iter().enumerate() {
            let symbol = if i == 0 {
                format!("{FORMAT}C{}{FORMAT}b{FORMAT}S14{symbol}", color.bits())
            } else {
                String::new()
            };

            lines.push(format!("{symbol}\t{}{part}", style(theme::TEXT_SECONDARY, KEY_SIZE)));
        }

        for part in wrap(&self.text, fit.value) {
            lines.push(format!("\t{}{part}", style(theme::TEXT, VALUE_SIZE)));
        }

        lines.push(gap());
        lines
    }
}


// Every change of the diff, sorted by key path.
pub(super) fn rows(diff: &SnapshotDiff) -> Vec<Row> {
    let registry = &diff.changes.registry;
    let mut rows = Vec::new();

    for key in &registry.keys_added {
        rows.push(Row::new(Filter::KeysAdded, Kind::Added, key, "(new key)".to_string()));
    }

    for key in &registry.keys_deleted {
        rows.push(Row::new(Filter::KeysDeleted, Kind::Deleted, key, "(deleted key)".to_string()));
    }

    for entry in &registry.values_added {
        let text = format!("{} = {}", name(&entry.name), describe(&entry.value));
        rows.push(Row::new(Filter::ValuesAdded, Kind::Added, &entry.key, text));
    }

    for entry in &registry.values_deleted {
        let text = format!("{} = {}", name(&entry.name), describe(&entry.value));
        rows.push(Row::new(Filter::ValuesDeleted, Kind::Deleted, &entry.key, text));
    }

    for change in &registry.values_modified {
        let text = format!(
            "{}: {}  →  {}",
            name(&change.name),
            describe(&change.before),
            describe(&change.after)
        );
        rows.push(Row::new(Filter::Modified, Kind::Modified, &change.key, text));
    }

    // Stable sort: the changes of a key stay in their order.
    rows.sort_by(|a, b| a.key.cmp(&b.key));
    rows
}


// Line shown when no change passes the filter and the search.
pub(super) fn empty() -> String {
    format!("\t{}(no changes match)", style(theme::TEXT_MUTED, VALUE_SIZE))
}


// Color, fixed-width font and size, at the start of a column.
fn style(color: Color, size: i32) -> String {
    format!("{FORMAT}C{}{FORMAT}f{FORMAT}S{size}", color.bits())
}


// A short empty line between two changes.
fn gap() -> String {
    format!("{FORMAT}S4{FORMAT}.")
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


// The default value of a key has an empty name.
fn name(name: &str) -> &str {
    if name.is_empty() { "(Default)" } else { name }
}


fn describe(value: &RegistryValue) -> String {
    match value {
        RegistryValue::Sz(text) => format!("Sz: {text}"),
        RegistryValue::ExpandSz(text) => format!("ExpandSz: {text}"),
        RegistryValue::MultiSz(list) => format!("MultiSz: {}", list.join(" | ")),
        RegistryValue::Dword(n) => format!("Dword: {n}"),
        RegistryValue::Qword(n) => format!("Qword: {n}"),
        RegistryValue::Binary(bytes) => format!("Binary: {}", preview(bytes)),
        RegistryValue::Other { kind, bytes } => format!("Type {kind}: {}", preview(bytes)),
    }
}


// First bytes in hex, and the total size if it was cut.
fn preview(bytes: &[u8]) -> String {
    let hex: String = bytes
        .iter()
        .take(BINARY_PREVIEW)
        .map(|b| format!("{b:02x}"))
        .collect();

    if bytes.len() > BINARY_PREVIEW {
        format!("{hex}… ({} bytes)", bytes.len())
    } else {
        hex
    }
}


// Tabs separate the columns, new lines end a line of the list and FORMAT
// starts a code: none of them may appear inside the text.
fn clean(text: &str) -> String {
    text.replace(['\t', '\r', '\n', FORMAT], " ")
}
