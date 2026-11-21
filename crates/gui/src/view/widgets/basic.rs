use fltk::button::Button;
use fltk::enums::{Align, FrameType};
use fltk::frame::Frame;
use fltk::input::Input;
use fltk::prelude::*;

use crate::theme;


pub(crate) const FIELD_HEIGHT: i32 = 30;


// Small grey text above a field.
pub(crate) fn caption(x: i32, y: i32, text: &str) {
    let mut frame = Frame::new(x, y, 300, 18, None);
    frame.set_label(text);
    frame.set_label_size(12);
    frame.set_label_color(theme::TEXT_SECONDARY);
    frame.set_align(Align::Left | Align::Inside);
}


// One-line text, such as a status or a result, cut if it does not fit.
pub(crate) fn line(x: i32, y: i32, w: i32, h: i32) -> Frame {
    let mut frame = Frame::new(x, y, w, h, None);
    frame.set_align(Align::Left | Align::Inside | Align::Clip);
    frame.set_label_size(12);
    frame
}


pub(crate) fn field(x: i32, y: i32, w: i32) -> Input {
    let mut input = Input::new(x, y, w, FIELD_HEIGHT, None);
    input.set_frame(theme::OUTLINE_BOX);
    input.set_color(theme::FIELD);
    input.set_text_font(theme::FONT_MONO);
    input.set_text_size(13);
    input.set_text_color(theme::TEXT);
    input
}


pub(crate) fn button(
    x: i32,
    y: i32,
    w: i32,
    label: &str
) -> Button {
    let mut button = Button::new(x, y, w, FIELD_HEIGHT, None);
    button.set_label(label);
    button.set_label_size(13);
    style_button(&mut button, false);
    button
}


// Primary: blue, for the next thing to do. Secondary: white with a border.
pub(crate) fn style_button(button: &mut Button, primary: bool) {
    if primary {
        button.set_frame(theme::FILLED_BOX);
        button.set_down_frame(theme::FILLED_BOX);
        button.set_color(theme::ACCENT);
        button.set_selection_color(theme::ACCENT_PRESSED);
        button.set_label_color(theme::ON_ACCENT);
    } else {
        button.set_frame(theme::OUTLINE_BOX);
        button.set_down_frame(theme::OUTLINE_BOX);
        button.set_color(theme::FIELD);
        button.set_selection_color(theme::PRESSED);
        button.set_label_color(theme::TEXT);
    }
}


// Selected: soft blue. Not selected: like a secondary button.
pub(crate) fn style_chip(button: &mut Button, selected: bool) {
    button.set_frame(theme::OUTLINE_BOX);
    button.set_down_frame(theme::OUTLINE_BOX);
    button.set_selection_color(theme::PRESSED);

    if selected {
        button.set_color(theme::ACCENT_SOFT);
        button.set_label_color(theme::ACCENT);
    } else {
        button.set_color(theme::FIELD);
        button.set_label_color(theme::TEXT_SECONDARY);
    }
}


pub(crate) fn separator(x: i32, y: i32, w: i32) {
    let mut line = Frame::new(x, y, w, 1, None);
    line.set_frame(FrameType::FlatBox);
    line.set_color(theme::BORDER);
}


pub(crate) fn set_enabled<W: WidgetExt>(widget: &mut W, enabled: bool) {
    if enabled { widget.activate(); } else { widget.deactivate(); }
}
