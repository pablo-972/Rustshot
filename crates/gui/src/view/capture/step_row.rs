use fltk::app::Sender;
use fltk::button::Button;
use fltk::frame::Frame;
use fltk::prelude::*;

use super::badge::Badge;
use super::layout::{
    BUTTON_WIDTH,
    BUTTON_X,
    FIELD_WIDTH,
    FIELD_X,
    MARGIN,
    ROW_BADGE,
    ROW_CAPTION,
    ROW_FIELD,
    ROW_HEIGHT,
    ROW_INFO,
    ROW_TOP,
};
use crate::theme;
use crate::types::Message;
use crate::view::widgets::{
    FileField,
    button,
    caption,
    line,
    set_enabled,
    style_button,
};


// One step of the workflow: number, file name, result and button.
pub(super) struct StepRow {
    badge: Badge,
    name: FileField,
    info: Frame,
    pub button: Button,
}

// What a row shows, decided by the window from the state.
pub(super) struct StepView {
    pub enabled: bool,
    pub openable: bool,
    pub primary: bool,
    pub running: Option<String>,
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
        let badge = Badge::new(MARGIN, top + ROW_BADGE, index + 1);
        caption(FIELD_X, top + ROW_CAPTION, caption_text);

        let mut name_field = FileField::new(
            FIELD_X,
            top + ROW_FIELD,
            FIELD_WIDTH,
            "Open an existing file",
            sender,
            open
        );
        name_field.set_value(name);

        let info = line(FIELD_X, top + ROW_INFO, FIELD_WIDTH, 16);
        let button = button(BUTTON_X, top + ROW_FIELD, BUTTON_WIDTH, "");

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

        self.badge.set_done(step.done.is_some());

        // Only how the step goes while it runs, or its result: no help texts.
        let (text, color) = match (step.running, step.done) {
            (Some(running), _) => (running, theme::TEXT_SECONDARY),
            (None, Some(info)) => (info, theme::SUCCESS),
            (None, None) => (String::new(), theme::TEXT_SECONDARY),
        };

        self.info.set_label(&text);
        self.info.set_label_color(color);
    }
}
