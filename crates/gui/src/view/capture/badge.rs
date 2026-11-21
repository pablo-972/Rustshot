use fltk::draw;
use fltk::enums::{Align, FrameType};
use fltk::frame::Frame;
use fltk::prelude::*;

use crate::theme;


// Round step number that turns into a check mark when the step is done.
pub(super) fn badge(x: i32, y: i32, number: i32) -> Frame {
    let mut frame = Frame::new(x, y, 26, 26, None);
    frame.set_frame(FrameType::NoBox);
    frame.set_color(theme::CARD);

    let number = number.to_string();
    frame.draw(move |f| {
        let (x, y, w, h) = (f.x(), f.y(), f.w(), f.h());
        let done = f.color() == theme::SUCCESS_BG;

        draw::set_draw_color(f.color());
        draw::draw_pie(x, y, w, h, 0.0, 360.0);

        if done {
            draw::set_draw_color(theme::SUCCESS);
            draw::set_line_style(draw::LineStyle::Solid, 2);
            draw::draw_line(x + 8, y + 13, x + 11, y + 17);
            draw::draw_line(x + 11, y + 17, x + 18, y + 9);
            draw::set_line_style(draw::LineStyle::Solid, 0);
        } else {
            draw::set_draw_color(theme::TEXT_SECONDARY);
            draw::set_font(theme::FONT_BOLD, 12);
            draw::draw_text2(&number, x, y, w, h, Align::Center);
        }
    });

    frame
}
