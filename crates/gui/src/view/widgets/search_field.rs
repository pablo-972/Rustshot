//! Search field with a magnifier and a help text inside.

use fltk::draw;
use fltk::enums::{Align, FrameType};
use fltk::frame::Frame;
use fltk::input::Input;
use fltk::prelude::*;

use super::basic::FIELD_HEIGHT;
use super::icons;
use crate::theme;


// Width taken by the magnifier at the left of the field.
const ICON_WIDTH: i32 = 30;


// Returns the input: it is the only part the window needs afterwards.
pub(crate) fn search_field(
    x: i32,
    y: i32,
    w: i32,
    help: &'static str
) -> Input {
    let mut outline = Frame::new(x, y, w, FIELD_HEIGHT, None);
    outline.set_frame(theme::OUTLINE_BOX);
    outline.set_color(theme::FIELD);

    let mut icon = Frame::new(x + 2, y + 2, ICON_WIDTH - 4, FIELD_HEIGHT - 4, None);
    icon.draw(|icon| icons::draw_centered(icons::search(theme::TEXT_SECONDARY), icon));

    let mut input = Input::new(
        x + ICON_WIDTH,
        y + 4,
        w - ICON_WIDTH - 6,
        FIELD_HEIGHT - 8,
        None
    );
    input.set_frame(FrameType::FlatBox);
    input.set_color(theme::FIELD);
    input.set_text_size(13);
    input.set_text_color(theme::TEXT);

    // FLTK inputs have no placeholder: the help text is drawn over the
    // input while it is empty.
    input.draw(move |input| {
        if input.value().is_empty() {
            draw::set_draw_color(theme::TEXT_MUTED);
            draw::set_font(theme::FONT, 13);
            draw::draw_text2(
                help,
                input.x() + 4,
                input.y(),
                input.w() - 4,
                input.h(),
                Align::Left | Align::Inside
            );
        }
    });

    input
}
