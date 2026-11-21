use fltk::app::Sender;
use fltk::button::Button;
use fltk::enums::FrameType;
use fltk::frame::Frame;
use fltk::input::Input;
use fltk::prelude::*;

use super::basic::{FIELD_HEIGHT, set_enabled};
use super::icons;
use crate::theme;
use crate::types::Message;


// Side of the square button inside the field.
const BUTTON_SIZE: i32 = 26;


pub(crate) struct FileField {
    input: Input,
    button: Button,
}


impl FileField {
    // FLTK inputs have no room for a button inside, so the field is made of
    // three pieces: the outline, the text and the button.
    pub(crate) fn new(
        x: i32,
        y: i32,
        w: i32,
        tooltip: &str,
        sender: Sender<Message>,
        message: Message
    ) -> Self {
        let mut outline = Frame::new(x, y, w, FIELD_HEIGHT, None);
        outline.set_frame(theme::OUTLINE_BOX);
        outline.set_color(theme::FIELD);

        let mut input = Input::new(
            x + 6,
            y + 4,
            w - BUTTON_SIZE - 14,
            FIELD_HEIGHT - 8,
            None
        );
        input.set_frame(FrameType::FlatBox);
        input.set_color(theme::FIELD);
        input.set_text_font(theme::FONT_MONO);
        input.set_text_size(13);
        input.set_text_color(theme::TEXT);

        let mut button = Button::new(
            x + w - BUTTON_SIZE - 2,
            y + 2,
            BUTTON_SIZE,
            FIELD_HEIGHT - 4,
            None
        );
        button.set_frame(FrameType::FlatBox);
        button.set_down_frame(FrameType::FlatBox);
        button.set_color(theme::FIELD);
        button.set_selection_color(theme::PRESSED);
        button.set_image(icons::folder_open());
        button.set_tooltip(tooltip);
        button.emit(sender, message);

        FileField { input, button }
    }

    pub(crate) fn value(&self) -> String {
        self.input.value().trim().to_string()
    }

    pub(crate) fn set_value(&mut self, text: &str) {
        self.input.set_value(text);
    }

    // The text and the button are enabled separately: a file can sometimes
    // be opened when the name cannot be changed, and the other way around.
    pub(crate) fn set_editable(&mut self, editable: bool) {
        set_enabled(&mut self.input, editable);
    }

    pub(crate) fn set_openable(&mut self, openable: bool) {
        set_enabled(&mut self.button, openable);
    }
}
