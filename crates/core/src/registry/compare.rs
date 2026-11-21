use std::collections::{BTreeMap, BTreeSet};

use crate::types::{
    RegistryDiff,
    RegistryKey,
    SnapshotData,
    ValueChange,
    ValueEntry,
};


pub(crate) fn compare(before: &SnapshotData, after: &SnapshotData) -> RegistryDiff {
    let mut diff = RegistryDiff::default();
    let empty = RegistryKey::default();

    // Keys that could not be read in either snapshot. What is below them is
    // unknown in one of the two, so it is left out instead of showing up as
    // added or deleted.
    let skipped: BTreeSet<&str> = before.inaccessible
        .iter()
        .chain(&after.inaccessible)
        .map(String::as_str)
        .collect();

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

    diff.not_compared = skipped.into_iter().map(str::to_string).collect();
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
fn inside(path: &str, skipped: &BTreeSet<&str>) -> bool {
    let mut current = path;

    loop {
        if skipped.contains(current) {
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
