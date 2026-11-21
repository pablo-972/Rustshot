//! Colors, fonts and box styles of the interface.

mod boxes;
mod colors;
mod fonts;
mod system;


pub(crate) use boxes::{
    FILLED_BOX,
    MENU_BOX,
    OUTLINE_BOX,
    register_boxes,
};
pub(crate) use colors::{
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
    ON_ACCENT,
    PRESSED,
    SUCCESS,
    SUCCESS_BG,
    TEXT,
    TEXT_MUTED,
    TEXT_SECONDARY,
    WARNING,
    set_mode,
    style_title_bar,
};
pub(crate) use fonts::{
    FONT,
    FONT_BOLD,
    FONT_MONO,
    load_fonts,
};
