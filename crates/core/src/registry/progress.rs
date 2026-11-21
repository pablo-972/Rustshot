use crate::types::SnapshotSummary;


const REPORT_EVERY: usize = 1000;


pub(super) struct Progress<'a> {
    captured: SnapshotSummary,
    report: &'a mut dyn FnMut(SnapshotSummary),
}

impl<'a> Progress<'a> {
    pub(super) fn new(report: &'a mut dyn FnMut(SnapshotSummary)) -> Self {
        Progress { captured: SnapshotSummary::default(), report }
    }

    pub(super) fn key(&mut self, values: usize) {
        self.captured.registry_keys += 1;
        self.captured.registry_values += values;

        if self.captured.registry_keys.is_multiple_of(REPORT_EVERY) {
            self.report();
        }
    }

    pub(super) fn inaccessible(&mut self) {
        self.captured.registry_inaccessible += 1;
    }

    pub(super) fn finish(&mut self) {
        self.report();
    }

    fn report(&mut self) {
        (self.report)(self.captured.clone());
    }
}
