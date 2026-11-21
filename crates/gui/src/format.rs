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


#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;


    #[test]
    fn counts_have_thousands_separators() {
        assert_eq!(format_count(0), "0");
        assert_eq!(format_count(999), "999");
        assert_eq!(format_count(1000), "1,000");
        assert_eq!(format_count(31507), "31,507");
        assert_eq!(format_count(1234567), "1,234,567");
    }

    #[test]
    fn file_name_is_the_last_part_of_the_path() {
        assert_eq!(file_name(Path::new(r"C:\analysis\diff.json")), "diff.json");
        assert_eq!(file_name(Path::new("before.json")), "before.json");
    }
}
