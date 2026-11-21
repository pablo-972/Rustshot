//! The light and dark values of the colors, and the one in use.

use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, Ordering};

use fltk::app;
use fltk::enums::Color;

use super::colors::{
    ACCENT,
    ACCENT_PRESSED,
    ACCENT_SOFT,
    ADDED_BG,
    BACKGROUND,
    BORDER,
    CARD,
    DANGER,
    DELETED_BG,
    FIELD,
    FOOTER,
    MODIFIED_BG,
    PRESSED,
    SUCCESS,
    SUCCESS_BG,
    TEXT,
    TEXT_MUTED,
    TEXT_SECONDARY,
    WARNING,
};
use super::system;
use crate::types::ThemeMode;


type Rgb = (u8, u8, u8);

// Light and dark value of each color. Borders and secondary texts keep a
// strong contrast with the backgrounds in both.
const PALETTE: [(Color, Rgb, Rgb); 23] = [
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
    (ADDED_BG, (238, 247, 234), (31, 45, 31)),
    (DELETED_BG, (251, 236, 235), (52, 33, 33)),
    (MODIFIED_BG, (252, 243, 227), (51, 41, 26)),

    // FLTK's own colors, for what it draws by itself: text and fields by
    // default, the selection, and the lines of scrollbars and of the
    // separators of the results list (as dark as our borders).
    (Color::Foreground, (0, 0, 0), (242, 242, 242)),
    (Color::Background2, (255, 255, 255), (44, 44, 44)),
    (Color::Selection, (181, 212, 244), (38, 79, 120)),
    (Color::Dark3, (155, 155, 155), (105, 105, 105)),
];

// Whether the dark palette is in use, for the title bars of the windows.
static DARK: AtomicBool = AtomicBool::new(false);


// Fills the palette with the light or the dark colors. The windows only need
// to be drawn again, and their title bars changed.
pub(crate) fn set_mode(mode: ThemeMode) {
    let dark = match mode {
        ThemeMode::System => system::dark(),
        ThemeMode::Light => false,
        ThemeMode::Dark => true,
    };
    DARK.store(dark, Ordering::Relaxed);

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
}


// The title bar of a shown window, light or dark like the rest.
pub(crate) fn style_title_bar(window: *mut c_void) {
    system::title_bar(window, DARK.load(Ordering::Relaxed));
}
