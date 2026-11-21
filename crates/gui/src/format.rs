use std::path::Path;


// 31507 -> "31,507"
pub(crate) fn format_count(n: usize) -> String {
    let digits = n.to_string();
    let mut out = String::new();

    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }

    out
}


// "C:\analysis\diff.json" -> "diff.json"
pub(crate) fn file_name(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default()
}
