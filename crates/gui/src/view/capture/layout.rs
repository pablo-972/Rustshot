//! Positions and sizes of the capture window, in pixels.

pub(super) const WIDTH: i32 = 560;
pub(super) const MARGIN: i32 = 20;

// Space between a field and the button at its right.
pub(super) const GAP: i32 = 10;

pub(super) const BUTTON_WIDTH: i32 = 140;
pub(super) const BUTTON_X: i32 = WIDTH - MARGIN - BUTTON_WIDTH;

// Menu bar, at the very top.
pub(super) const MENU_HEIGHT: i32 = 24;

// Output folder, below the menu bar.
pub(super) const OUTPUT_CAPTION_TOP: i32 = MENU_HEIGHT + 14;
pub(super) const OUTPUT_TOP: i32 = MENU_HEIGHT + 34;
pub(super) const OUTPUT_WIDTH: i32 = BUTTON_X - MARGIN - GAP;

// Fields of the steps, at the right of their number.
pub(super) const FIELD_X: i32 = 60;
pub(super) const FIELD_WIDTH: i32 = BUTTON_X - FIELD_X - GAP;

// Line between the output folder and the steps, with some room below it.
pub(super) const SEPARATOR_TOP: i32 = MENU_HEIGHT + 80;
pub(super) const ROW_TOP: i32 = SEPARATOR_TOP + 14;
pub(super) const ROW_HEIGHT: i32 = 84;

// Parts of a step, from the top of its row.
pub(super) const ROW_CAPTION: i32 = 12;
pub(super) const ROW_FIELD: i32 = 32;
pub(super) const ROW_BADGE: i32 = 34;
pub(super) const ROW_INFO: i32 = 64;

// The last row is taller: it has Compare and View results.
pub(super) const DIFF_ROW_HEIGHT: i32 = 104;

// View results, below Compare: its height (30) and a gap of 4.
pub(super) const RESULTS_TOP: i32 = ROW_TOP + 2 * ROW_HEIGHT + ROW_FIELD + 34;

// Space between the last step and the bottom bar.
pub(super) const FOOTER_GAP: i32 = 26;

pub(super) const FOOTER_TOP: i32 = ROW_TOP + 2 * ROW_HEIGHT + DIFF_ROW_HEIGHT + FOOTER_GAP;
pub(super) const FOOTER_PADDING: i32 = 8;
pub(super) const HEIGHT: i32 = FOOTER_TOP + 46;
