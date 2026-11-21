//! Menu bar: File, Options and Help.

use fltk::app::{self, Sender};
use fltk::enums::{Color, FrameType, Shortcut};
use fltk::menu::{MenuBar, MenuFlag};
use fltk::prelude::*;

use super::layout::{MENU_HEIGHT, WIDTH};
use crate::theme;
use crate::types::{Message, Step, ThemeMode};
use crate::view::widgets::separator;


// Paths of the items. FLTK finds the items by them, with the '&' included.
const OPEN_FIRST: &str = "&File/Open &shot/&First…";
const OPEN_SECOND: &str = "&File/Open &shot/&Second…";
const OPEN_DIFF: &str = "&File/Open &diff…";
const EXIT: &str = "&File/E&xit";
const ABOUT: &str = "&Help/&About Rustshot";

const THEMES: [(ThemeMode, &str); 3] = [
    (ThemeMode::System, "&Options/&Theme/&System"),
    (ThemeMode::Light, "&Options/&Theme/&Light"),
    (ThemeMode::Dark, "&Options/&Theme/&Dark"),
];


pub(super) struct Menu {
    bar: MenuBar,
}

// What the menu shows, decided by the window from the state.
pub(super) struct MenuView {
    pub open_first: bool,
    pub open_second: bool,
    pub open_diff: bool,
    pub theme: ThemeMode,
}

impl Menu {
    // The items of File send the same messages as the buttons of the fields.
    pub(super) fn new(sender: Sender<Message>) -> Self {
        let mut bar = MenuBar::new(0, 0, WIDTH, MENU_HEIGHT, None);
        bar.set_frame(FrameType::FlatBox);
        bar.set_down_frame(FrameType::FlatBox);
        bar.set_menu_frame(theme::MENU_BOX);
        bar.set_color(theme::BACKGROUND);
        bar.set_selection_color(Color::Selection);
        bar.set_text_color(theme::TEXT);
        bar.set_text_size(12);

        let none = Shortcut::None;
        bar.add_emit(OPEN_FIRST, none, MenuFlag::Normal, sender, Message::OpenShot(Step::First));
        bar.add_emit(OPEN_SECOND, none, MenuFlag::Normal, sender, Message::OpenShot(Step::Second));
        bar.add_emit(OPEN_DIFF, none, MenuFlag::MenuDivider, sender, Message::OpenDiff);
        bar.add(EXIT, none, MenuFlag::Normal, |_| app::quit());

        for (mode, path) in THEMES {
            bar.add_emit(path, none, MenuFlag::Radio, sender, Message::Theme(mode));
        }

        bar.add_emit(ABOUT, none, MenuFlag::Normal, sender, Message::About);

        // Line between the menu bar and the rest of the window.
        separator(0, MENU_HEIGHT, WIDTH);

        Menu { bar }
    }

    pub(super) fn render(&mut self, view: MenuView) {
        self.set_active(OPEN_FIRST, view.open_first);
        self.set_active(OPEN_SECOND, view.open_second);
        self.set_active(OPEN_DIFF, view.open_diff);

        for (mode, path) in THEMES {
            if let Some(mut item) = self.bar.find_item(path) {
                if mode == view.theme { item.set(); } else { item.clear(); }
            }
        }

        self.bar.redraw();
    }

    fn set_active(&mut self, path: &str, active: bool) {
        if let Some(mut item) = self.bar.find_item(path) {
            if active { item.activate(); } else { item.deactivate(); }
        }
    }
}
