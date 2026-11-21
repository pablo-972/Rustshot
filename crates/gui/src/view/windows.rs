use fltk::app::Sender;

use super::capture::CaptureWindow;
use super::results::ResultsWindow;
use crate::types::Message;


pub(crate) struct Windows {
    pub capture: CaptureWindow,
    pub results: ResultsWindow,
}

impl Windows {
    pub(crate) fn new(sender: Sender<Message>, output: &str) -> Self {
        let capture = CaptureWindow::new(sender, output);
        let results = ResultsWindow::new(sender);

        Windows { capture, results }
    }

    pub(crate) fn restyle(&mut self) {
        self.capture.restyle();
        self.results.restyle();
    }
}
