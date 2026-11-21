//! The colors of the interface.
//!
//! Each color is a slot of the FLTK color map, filled by the palette with its
//! light or dark value, so the colors can stay constants.

use fltk::enums::Color;


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

// Pale backgrounds of the changes in the results list. FLTK only has 16 free
// slots (16 to 31), so these take slots of its color cube that it does not
// name or use.
pub(crate) const ADDED_BG: Color = Color::from_rgbi(57);
pub(crate) const DELETED_BG: Color = Color::from_rgbi(58);
pub(crate) const MODIFIED_BG: Color = Color::from_rgbi(59);
