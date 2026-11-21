//! The changes of a diff as rows: what each one says and how it is found.

use rustshot_core::{RegistryValue, SnapshotDiff};

use super::list::FORMAT;
use crate::types::Filter;


// Bytes shown of a binary value before cutting it.
const BINARY_PREVIEW: usize = 32;


// One change of the diff, ready to be shown.
pub(super) struct Row {
    pub filter: Filter,
    pub key: String,
    pub text: String,
    search: String,
}

impl Row {
    fn new(filter: Filter, key: &str, text: String) -> Self {
        let key = clean(key);
        let text = clean(&text);

        Row {
            filter,
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
}


// Every change of the diff, sorted by key path.
pub(super) fn rows(diff: &SnapshotDiff) -> Vec<Row> {
    let registry = &diff.changes.registry;
    let mut rows = Vec::new();

    for key in &registry.keys_added {
        rows.push(Row::new(Filter::KeysAdded, key, "(new key)".to_string()));
    }

    for key in &registry.keys_deleted {
        rows.push(Row::new(Filter::KeysDeleted, key, "(deleted key)".to_string()));
    }

    for entry in &registry.values_added {
        let text = format!("{} = {}", name(&entry.name), describe(&entry.value));
        rows.push(Row::new(Filter::ValuesAdded, &entry.key, text));
    }

    for entry in &registry.values_deleted {
        let text = format!("{} = {}", name(&entry.name), describe(&entry.value));
        rows.push(Row::new(Filter::ValuesDeleted, &entry.key, text));
    }

    for change in &registry.values_modified {
        let text = format!(
            "{}: {}  →  {}",
            name(&change.name),
            describe(&change.before),
            describe(&change.after)
        );
        rows.push(Row::new(Filter::Modified, &change.key, text));
    }

    // Stable sort: the changes of a key stay in their order.
    rows.sort_by(|a, b| a.key.cmp(&b.key));
    rows
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


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn row_matches_its_filter_and_all() {
        let row = Row::new(Filter::KeysAdded, r"HKLM\Run", "(new key)".to_string());

        assert!(row.matches(Filter::All, ""));
        assert!(row.matches(Filter::KeysAdded, ""));
        assert!(!row.matches(Filter::KeysDeleted, ""));
    }

    #[test]
    fn search_looks_in_key_and_value_ignoring_case() {
        let row = Row::new(Filter::ValuesAdded, r"HKLM\Run", "Updater = Sz: C:\\upd.exe".into());

        assert!(row.matches(Filter::All, "run"));
        assert!(row.matches(Filter::All, "upd.exe"));
        assert!(!row.matches(Filter::All, "notepad"));
    }

    #[test]
    fn text_is_cleaned_of_tabs_new_lines_and_format_codes() {
        let row = Row::new(Filter::ValuesAdded, "a\tb", format!("c\nd{FORMAT}e"));

        assert_eq!(row.key, "a b");
        assert_eq!(row.text, "c d e");
    }
}
