// Positions and sizes of the capture window, in pixels.

pub(super) const WIDTH: i32 = 560;
pub(super) const MARGIN: i32 = 20;

pub(super) const BUTTON_WIDTH: i32 = 140;
pub(super) const BUTTON_X: i32 = WIDTH - MARGIN - BUTTON_WIDTH;

pub(super) const FIELD_X: i32 = 60;
pub(super) const FIELD_WIDTH: i32 = BUTTON_X - FIELD_X - 10;

pub(super) const ROW_TOP: i32 = 82;
pub(super) const ROW_HEIGHT: i32 = 84;

// The last row is taller: it has Compare and View results.
pub(super) const DIFF_ROW_HEIGHT: i32 = 104;

pub(super) const FOOTER_TOP: i32 = ROW_TOP + 2 * ROW_HEIGHT + DIFF_ROW_HEIGHT + 10;
pub(super) const HEIGHT: i32 = FOOTER_TOP + 46;
