//! Fonts of the interface. FLTK's built-in fonts are replaced by the ones of
//! Windows, which exist from Windows 7 on.

use fltk::app;
use fltk::enums::Font;


pub(crate) const FONT: Font = Font::Helvetica;
pub(crate) const FONT_BOLD: Font = Font::HelveticaBold;
pub(crate) const FONT_MONO: Font = Font::Courier;


// Set once at start.
pub(crate) fn load_fonts() {
    Font::set_font(FONT, "Segoe UI");
    Font::set_font(FONT_BOLD, "Segoe UI Semibold");
    Font::set_font(FONT_MONO, "Consolas");

    app::set_font_size(13);
}
