use crate::types::SnapshotSummary;


const REPORT_EVERY: usize = 1000;


pub(super) struct Progress<'a> {
    values: usize,
    report: &'a mut dyn FnMut(SnapshotSummary),
}

impl<'a> Progress<'a> {
    pub(super) fn new(report: &'a mut dyn FnMut(SnapshotSummary)) -> Self {
        Progress { values: 0, report }
    }

    // A key captured with its values. `keys` is the total captured so far.
    pub(super) fn add(&mut self, keys: usize, values: usize) {
        self.values += values;

        if keys.is_multiple_of(REPORT_EVERY) {
            self.report(keys);
        }
    }

    pub(super) fn finish(&mut self, keys: usize) {
        self.report(keys);
    }

    fn report(&mut self, keys: usize) {
        (self.report)(SnapshotSummary {
            registry_keys: keys,
            registry_values: self.values,
        });
    }
}
