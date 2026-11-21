//! Colors, fonts and box styles of the interface.

mod system;

use fltk::app;
use fltk::draw;
use fltk::enums::{Color, Font, FrameType};

pub(crate) use system::dark_title_bar;


// Each color is a free slot of the FLTK color map. apply() fills the slots
// with the light or the dark palette, so the colors can stay constants.
pub(crate) const BACKGROUND: Color = Color::from_rgbi(16);
pub(crate) const FOOTER: Color = Color::from_rgbi(17);
pub(crate) const CARD: Color = Color::from_rgbi(18);
pub(crate) const FIELD: Color = Color::from_rgbi(19);
pub(crate) const PRESSED: Color = Color::from_rgbi(20);
pub(crate) const BORDER: Color = Color::from_rgbi(21);

pub(crate) const TEXT: Color = Color::from_rgbi(22);
pub(crate) const TEXT_SECONDARY: Color = Color::from_rgbi(23);
pub(crate) const TEXT_MUTED: Color = Color::from_rgbi(24);

pub(crate) const ACCENT: Color = Color::from_rgbi(25);
pub(crate) const ACCENT_PRESSED: Color = Color::from_rgbi(26);
pub(crate) const ACCENT_SOFT: Color = Color::from_rgbi(27);
pub(crate) const ON_ACCENT: Color = Color::from_rgb(255, 255, 255);

pub(crate) const SUCCESS: Color = Color::from_rgbi(28);
pub(crate) const SUCCESS_BG: Color = Color::from_rgbi(29);
pub(crate) const DANGER: Color = Color::from_rgbi(30);
pub(crate) const WARNING: Color = Color::from_rgbi(31);


type Rgb = (u8, u8, u8);

// Light and dark value of each color. Borders and secondary texts keep a
// strong contrast with the backgrounds in both.
const PALETTE: [(Color, Rgb, Rgb); 20] = [
    (BACKGROUND, (250, 250, 249), (32, 32, 32)),
    (FOOTER, (241, 240, 236), (40, 40, 40)),
    (CARD, (243, 242, 239), (52, 52, 52)),
    (FIELD, (255, 255, 255), (44, 44, 44)),
    (PRESSED, (236, 235, 231), (60, 60, 60)),
    (BORDER, (155, 155, 155), (105, 105, 105)),
    (TEXT, (0, 0, 0), (242, 242, 242)),
    (TEXT_SECONDARY, (55, 55, 55), (200, 200, 200)),
    (TEXT_MUTED, (95, 95, 95), (155, 155, 155)),
    (ACCENT, (37, 99, 199), (52, 116, 214)),
    (ACCENT_PRESSED, (28, 78, 160), (40, 94, 180)),
    (ACCENT_SOFT, (222, 234, 250), (30, 46, 74)),
    (SUCCESS, (25, 105, 25), (120, 200, 110)),
    (SUCCESS_BG, (222, 238, 205), (36, 66, 34)),
    (DANGER, (175, 25, 25), (240, 115, 110)),
    (WARNING, (150, 80, 0), (232, 168, 70)),

    // FLTK's own colors, for what it draws by itself: text and fields by
    // default, the selection, and the lines of scrollbars and of the
    // separators of the results list (as dark as our borders).
    (Color::Foreground, (0, 0, 0), (242, 242, 242)),
    (Color::Background2, (255, 255, 255), (44, 44, 44)),
    (Color::Selection, (181, 212, 244), (38, 79, 120)),
    (Color::Dark3, (155, 155, 155), (105, 105, 105)),
];


// Box types the base scheme does not use, redrawn with our own style.
pub(crate) const OUTLINE_BOX: FrameType = FrameType::GleamUpBox;
pub(crate) const FILLED_BOX: FrameType = FrameType::GleamThinUpBox;

pub(crate) const FONT: Font = Font::Helvetica;
pub(crate) const FONT_BOLD: Font = Font::HelveticaBold;
pub(crate) const FONT_MONO: Font = Font::Courier;

const RADIUS: i32 = 6;


pub(crate) fn apply() {
    let dark = system::dark();
    let pick = |light_rgb, dark_rgb| if dark { dark_rgb } else { light_rgb };

    // The background also recalculates FLTK's greys, so it goes before the
    // palette, which changes some of them.
    let (_, light_rgb, dark_rgb) = PALETTE[0];
    let (r, g, b) = pick(light_rgb, dark_rgb);
    app::background(r, g, b);

    for (slot, light_rgb, dark_rgb) in PALETTE {
        let (r, g, b) = pick(light_rgb, dark_rgb);
        app::set_color(slot, r, g, b);
    }

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
