use std::collections::{BTreeMap, BTreeSet};

use crate::types::{
    NotCompared,
    RegistryDiff,
    RegistryKey,
    SnapshotData,
    UnreadableIn,
    ValueChange,
    ValueEntry,
};


pub(crate) fn compare(before: &SnapshotData, after: &SnapshotData) -> RegistryDiff {
    let mut diff = RegistryDiff::default();
    let empty = RegistryKey::default();

    // Keys that could not be read in either snapshot, and in which. What is
    // below them is unknown in one of the two, so it is left out instead of
    // showing up as added or deleted.
    let mut skipped = BTreeMap::new();

    for key in &before.inaccessible {
        skipped.insert(key.as_str(), UnreadableIn::Before);
    }

    for key in &after.inaccessible {
        skipped
            .entry(key.as_str())
            .and_modify(|unreadable_in| *unreadable_in = UnreadableIn::Both)
            .or_insert(UnreadableIn::After);
    }

    for path in union(&before.registry, &after.registry) {
        if inside(path, &skipped) {
            continue;
        }

        let old = before.registry.get(path);
        let new = after.registry.get(path);

        match (old, new) {
            (Some(_), None) => diff.keys_deleted.push(path.clone()),
            (None, Some(_)) => diff.keys_added.push(path.clone()),
            _ => {}
        }

        // A missing key counts as empty, so the values of added and
        // deleted keys show up as added and deleted values.
        compare_values(
            path,
            old.unwrap_or(&empty),
            new.unwrap_or(&empty),
            &mut diff,
        );
    }

    diff.not_compared = skipped
        .into_iter()
        .map(|(key, unreadable_in)| NotCompared { key: key.to_string(), unreadable_in })
        .collect();

    diff
}


fn compare_values(
    path: &str,
    old: &RegistryKey,
    new: &RegistryKey,
    diff: &mut RegistryDiff,
) {
    for name in union(&old.values, &new.values) {
        match (old.values.get(name), new.values.get(name)) {
            (Some(value), None) => diff.values_deleted.push(ValueEntry {
                key: path.to_string(),
                name: name.clone(),
                value: value.clone(),
            }),

            (None, Some(value)) => diff.values_added.push(ValueEntry {
                key: path.to_string(),
                name: name.clone(),
                value: value.clone(),
            }),

            (Some(before), Some(after)) if before != after => {
                diff.values_modified.push(ValueChange {
                    key: path.to_string(),
                    name: name.clone(),
                    before: before.clone(),
                    after: after.clone(),
                })
            }

            _ => {}
        }
    }
}

// Whether `path` is one of `skipped` or is below one of them.
fn inside(path: &str, skipped: &BTreeMap<&str, UnreadableIn>) -> bool {
    let mut current = path;

    loop {
        if skipped.contains_key(current) {
            return true;
        }

        match current.rsplit_once('\\') {
            Some((parent, _)) => current = parent,
            None => return false,
        }
    }
}

// Keys of both maps, sorted and without duplicates.
fn union<'a, V>(
    a: &'a BTreeMap<String, V>,
    b: &'a BTreeMap<String, V>,
) -> BTreeSet<&'a String> {
    a.keys().chain(b.keys()).collect()
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::RegistryValue;


    fn key(values: &[(&str, u32)]) -> RegistryKey {
        let values = values
            .iter()
            .map(|(name, n)| (name.to_string(), RegistryValue::Dword(*n)))
            .collect();

        RegistryKey { values }
    }

    fn data(keys: &[(&str, RegistryKey)], inaccessible: &[&str]) -> SnapshotData {
        SnapshotData {
            registry: keys.iter().map(|(path, key)| (path.to_string(), key.clone())).collect(),
            inaccessible: inaccessible.iter().map(|path| path.to_string()).collect(),
        }
    }

    #[test]
    fn same_snapshots_have_no_changes() {
        let snapshot = data(&[(r"HKLM\A", key(&[("x", 1)]))], &[]);
        let diff = compare(&snapshot, &snapshot);

        assert_eq!(diff, RegistryDiff::default());
    }

    #[test]
    fn added_key_shows_its_values_as_added() {
        let before = data(&[], &[]);
        let after = data(&[(r"HKLM\Run", key(&[("evil", 1)]))], &[]);
        let diff = compare(&before, &after);

        assert_eq!(diff.keys_added, [r"HKLM\Run"]);
        assert_eq!(diff.values_added.len(), 1);
        assert_eq!(diff.values_added[0].name, "evil");
        assert!(diff.keys_deleted.is_empty() && diff.values_deleted.is_empty());
    }

    #[test]
    fn deleted_key_shows_its_values_as_deleted() {
        let before = data(&[(r"HKLM\Old", key(&[("a", 1), ("b", 2)]))], &[]);
        let after = data(&[], &[]);
        let diff = compare(&before, &after);

        assert_eq!(diff.keys_deleted, [r"HKLM\Old"]);
        assert_eq!(diff.values_deleted.len(), 2);
        assert!(diff.keys_added.is_empty() && diff.values_added.is_empty());
    }

    #[test]
    fn values_are_added_deleted_and_modified() {
        let before = data(&[(r"HKLM\K", key(&[("same", 1), ("gone", 2), ("changed", 3)]))], &[]);
        let after = data(&[(r"HKLM\K", key(&[("same", 1), ("new", 4), ("changed", 5)]))], &[]);
        let diff = compare(&before, &after);

        assert!(diff.keys_added.is_empty() && diff.keys_deleted.is_empty());
        assert_eq!(diff.values_added[0].name, "new");
        assert_eq!(diff.values_deleted[0].name, "gone");
        assert_eq!(diff.values_modified.len(), 1);
        assert_eq!(diff.values_modified[0].name, "changed");
        assert_eq!(diff.values_modified[0].before, RegistryValue::Dword(3));
        assert_eq!(diff.values_modified[0].after, RegistryValue::Dword(5));
    }

    #[test]
    fn inaccessible_keys_are_not_compared() {
        // Unreadable in the first shot, readable in the second: without the
        // skip, the whole subtree would show up as added.
        let before = data(&[(r"HKLM\SAM", key(&[]))], &[r"HKLM\SAM\SAM"]);
        let after = data(
            &[
                (r"HKLM\SAM", key(&[])),
                (r"HKLM\SAM\SAM", key(&[("x", 1)])),
                (r"HKLM\SAM\SAM\Domains", key(&[("y", 2)])),
            ],
            &[],
        );
        let diff = compare(&before, &after);

        assert!(diff.keys_added.is_empty());
        assert!(diff.values_added.is_empty());
        assert_eq!(diff.not_compared.len(), 1);
        assert_eq!(diff.not_compared[0].key, r"HKLM\SAM\SAM");
        assert_eq!(diff.not_compared[0].unreadable_in, UnreadableIn::Before);
    }

    #[test]
    fn not_compared_says_in_which_shot_it_was_unreadable() {
        let before = data(&[], &[r"HKLM\A", r"HKLM\C"]);
        let after = data(&[], &[r"HKLM\B", r"HKLM\C"]);
        let found: Vec<_> = compare(&before, &after)
            .not_compared
            .into_iter()
            .map(|entry| (entry.key, entry.unreadable_in))
            .collect();

        assert_eq!(found, [
            (r"HKLM\A".to_string(), UnreadableIn::Before),
            (r"HKLM\B".to_string(), UnreadableIn::After),
            (r"HKLM\C".to_string(), UnreadableIn::Both),
        ]);
    }

    #[test]
    fn inside_checks_the_path_and_its_parents() {
        let skipped = BTreeMap::from([(r"HKLM\SAM\SAM", UnreadableIn::Both)]);

        assert!(inside(r"HKLM\SAM\SAM", &skipped));
        assert!(inside(r"HKLM\SAM\SAM\Domains\Account", &skipped));
        assert!(!inside(r"HKLM\SAM", &skipped));
        assert!(!inside(r"HKLM\SAM\SAMX", &skipped));
        assert!(!inside("HKLM", &skipped));
    }

    #[test]
    fn union_has_the_keys_of_both_once() {
        let a = BTreeMap::from([("x".to_string(), 1), ("y".to_string(), 2)]);
        let b = BTreeMap::from([("y".to_string(), 3), ("z".to_string(), 4)]);
        let keys: Vec<_> = union(&a, &b).into_iter().map(String::as_str).collect();

        assert_eq!(keys, ["x", "y", "z"]);
    }
}
