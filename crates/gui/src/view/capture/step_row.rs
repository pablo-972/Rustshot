use fltk::app::Sender;
use fltk::button::Button;
use fltk::frame::Frame;
use fltk::prelude::*;

use super::layout::{
    BUTTON_WIDTH,
    BUTTON_X,
    FIELD_WIDTH,
    FIELD_X,
    MARGIN,
    ROW_HEIGHT,
    ROW_TOP,
};
use crate::theme;
use crate::types::Message;
use crate::view::file_field::FileField;
use crate::view::widgets::{
    badge,
    button,
    caption,
    line,
    set_enabled,
    style_button,
};


// One step of the workflow: number, file name, result and button.
pub(super) struct StepRow {
    badge: Frame,
    name: FileField,
    info: Frame,
    pub button: Button,
}


// What a row shows, decided by the window from the state.
pub(super) struct StepView {
    pub enabled: bool,
    pub openable: bool,
    pub primary: bool,
    pub running: bool,
    pub label: &'static str,
    pub done: Option<String>,
}


impl StepRow {
    pub(super) fn new(
        index: i32,
        caption_text: &'static str,
        name: &str,
        sender: Sender<Message>,
        open: Message
    ) -> Self {
        let top = ROW_TOP + index * ROW_HEIGHT;
        let badge = badge(MARGIN, top + 34, index + 1);
        caption(FIELD_X, top + 12, caption_text);

        let mut name_field = FileField::new(
            FIELD_X,
            top + 32,
            FIELD_WIDTH,
            "Open an existing file",
            sender,
            open
        );
        name_field.set_value(name);

        let info = line(FIELD_X, top + 64, FIELD_WIDTH, 16);
        let button = button(BUTTON_X, top + 32, BUTTON_WIDTH, "");

        StepRow {
            badge,
            name: name_field,
            info,
            button,
        }
    }

    pub(super) fn name(&self) -> String {
        self.name.value()
    }

    pub(super) fn set_name(&mut self, name: &str) {
        self.name.set_value(name);
    }

    pub(super) fn render(&mut self, step: StepView) {
        set_enabled(&mut self.button, step.enabled);
        self.name.set_editable(step.enabled);
        self.name.set_openable(step.openable);
        self.button.set_label(step.label);
        style_button(&mut self.button, step.primary && step.enabled);

        let done = step.done.is_some();
        self.badge.set_color(if done { theme::SUCCESS_BG } else { theme::CARD });

        // Only the result of the step, or that it is running: no help texts.
        let (text, color) = match (step.running, step.done) {
            (true, _) => ("Working…".to_string(), theme::TEXT_SECONDARY),
            (false, Some(info)) => (info, theme::SUCCESS),
            (false, None) => (String::new(), theme::TEXT_SECONDARY),
        };

        self.info.set_label(&text);
        self.info.set_label_color(color);
    }
}
