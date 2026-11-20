use fltk::app::Sender;

use super::capture::CaptureWindow;
use super::results::ResultsWindow;
use crate::types::Message;


// Both windows of the application.
pub(crate) struct Windows {
    pub capture: CaptureWindow,
    pub results: ResultsWindow,
}


impl Windows {
    pub(crate) fn new(sender: Sender<Message>, output: &str) -> Self {
        // FLTK adds new widgets to the last window created, so each window is
        // finished before the next one starts.
        let capture = CaptureWindow::new(sender, output);
        let results = ResultsWindow::new(sender);

        Windows { capture, results }
    }
}
