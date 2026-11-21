use std::collections::BTreeMap;

use super::diff::union;
use crate::types::{
    RegistryDiff,
    RegistryKey,
    ValueChange,
    ValueEntry,
};


pub(super) fn compare(
    before: &BTreeMap<String, RegistryKey>,
    after: &BTreeMap<String, RegistryKey>,
) -> RegistryDiff {
    let mut diff = RegistryDiff::default();
    let empty = RegistryKey::default();

    for path in union(before, after) {
        let old = before.get(path);
        let new = after.get(path);

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
