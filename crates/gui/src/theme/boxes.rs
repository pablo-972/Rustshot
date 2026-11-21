//! Box types the base scheme does not use, redrawn with our own style.

use fltk::app;
use fltk::draw;
use fltk::enums::{Color, FrameType};

use super::colors::BORDER;


pub(crate) const OUTLINE_BOX: FrameType = FrameType::GleamUpBox;
pub(crate) const FILLED_BOX: FrameType = FrameType::GleamThinUpBox;
pub(crate) const MENU_BOX: FrameType = FrameType::GleamDownBox;

const RADIUS: i32 = 6;


// Set once at start. Without the dotted focus box of FLTK, which does not
// fit the rounded boxes.
pub(crate) fn register_boxes() {
    app::set_visible_focus(false);

    app::set_frame_type_cb(OUTLINE_BOX, draw_outline_box, 1, 1, 2, 2);
    app::set_frame_type_cb(FILLED_BOX, draw_filled_box, 0, 0, 0, 0);
    app::set_frame_type_cb(MENU_BOX, draw_menu_box, 1, 1, 2, 2);
}


fn draw_outline_box(x: i32, y: i32, w: i32, h: i32, color: Color) {
    draw::draw_rbox(x, y, w, h, RADIUS, true, color);
    draw::draw_rbox(x, y, w, h, RADIUS, false, BORDER);
}


fn draw_filled_box(x: i32, y: i32, w: i32, h: i32, color: Color) {
    draw::draw_rbox(x, y, w, h, RADIUS, true, color);
}


// Square, with a border: the menus that drop down from the menu bar.
fn draw_menu_box(x: i32, y: i32, w: i32, h: i32, color: Color) {
    draw::draw_rect_fill(x, y, w, h, color);
    draw::draw_rect_with_color(x, y, w, h, BORDER);
}
