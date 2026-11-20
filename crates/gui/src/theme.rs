//! Colors, fonts and box styles of the interface.

use fltk::app;
use fltk::draw;
use fltk::enums::{Color, Font, FrameType};


pub(crate) const BACKGROUND: Color = Color::from_rgb(250, 250, 249);
pub(crate) const FOOTER: Color = Color::from_rgb(241, 240, 236);
pub(crate) const CARD: Color = Color::from_rgb(243, 242, 239);
pub(crate) const FIELD: Color = Color::from_rgb(255, 255, 255);
pub(crate) const PRESSED: Color = Color::from_rgb(236, 235, 231);
pub(crate) const BORDER: Color = Color::from_rgb(214, 212, 205);

pub(crate) const TEXT: Color = Color::from_rgb(31, 30, 29);
pub(crate) const TEXT_SECONDARY: Color = Color::from_rgb(107, 105, 99);
pub(crate) const TEXT_MUTED: Color = Color::from_rgb(150, 148, 141);

pub(crate) const ACCENT: Color = Color::from_rgb(37, 99, 199);
pub(crate) const ACCENT_PRESSED: Color = Color::from_rgb(28, 78, 160);
pub(crate) const ACCENT_SOFT: Color = Color::from_rgb(222, 234, 250);
pub(crate) const ON_ACCENT: Color = Color::from_rgb(255, 255, 255);

pub(crate) const SUCCESS: Color = Color::from_rgb(59, 109, 17);
pub(crate) const SUCCESS_BG: Color = Color::from_rgb(222, 238, 205);
pub(crate) const DANGER: Color = Color::from_rgb(163, 45, 45);
pub(crate) const WARNING: Color = Color::from_rgb(160, 98, 10);


// Box types the base scheme does not use, redrawn with our own style.
pub(crate) const OUTLINE_BOX: FrameType = FrameType::GleamUpBox;
pub(crate) const FILLED_BOX: FrameType = FrameType::GleamThinUpBox;

pub(crate) const FONT: Font = Font::Helvetica;
pub(crate) const FONT_BOLD: Font = Font::HelveticaBold;
pub(crate) const FONT_MONO: Font = Font::Courier;

const RADIUS: i32 = 6;


pub(crate) fn apply() {
    app::background(250, 250, 249);
    app::background2(255, 255, 255);
    app::foreground(31, 30, 29);
    app::set_selection_color(181, 212, 244);
    app::set_visible_focus(false);
    app::set_font_size(13);

    app::set_frame_type_cb(OUTLINE_BOX, draw_outline_box, 1, 1, 2, 2);
    app::set_frame_type_cb(FILLED_BOX, draw_filled_box, 0, 0, 0, 0);

    // Both fonts exist from Windows 7 on.
    Font::set_font(FONT, "Segoe UI");
    Font::set_font(FONT_BOLD, "Segoe UI Semibold");
    Font::set_font(FONT_MONO, "Consolas");
}


fn draw_outline_box(x: i32, y: i32, w: i32, h: i32, color: Color) {
    draw::draw_rbox(x, y, w, h, RADIUS, true, color);
    draw::draw_rbox(x, y, w, h, RADIUS, false, BORDER);
}


fn draw_filled_box(x: i32, y: i32, w: i32, h: i32, color: Color) {
    draw::draw_rbox(x, y, w, h, RADIUS, true, color);
}
