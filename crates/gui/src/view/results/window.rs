use fltk::app::Sender;
use fltk::browser::HoldBrowser;
use fltk::button::Button;
use fltk::enums::CallbackTrigger;
use fltk::input::Input;
use fltk::prelude::*;
use fltk::window::Window;

use super::layout::{
    CHIP_GAP,
    CHIP_WIDTH,
    HEIGHT,
    LIST_PADDING,
    LIST_TOP,
    MARGIN,
    SEARCH_TOP,
    SYMBOL_WIDTH,
    WIDTH,
};
use super::rows::{FORMAT, Fit, Row, empty, rows};
use crate::theme;
use crate::types::{DiffResult, Filter, Message};
use crate::view::format::{file_name, format_count};
use crate::view::widgets::{button, field, style_chip};


// The filters, in the order of their buttons.
const FILTERS: [(Filter, &str); 6] = [
    (Filter::All, "All"),
    (Filter::KeysAdded, "Keys added"),
    (Filter::KeysDeleted, "Keys deleted"),
    (Filter::ValuesAdded, "Values added"),
    (Filter::ValuesDeleted, "Values deleted"),
    (Filter::Modified, "Modified"),
];


pub(crate) struct ResultsWindow {
    window: Window,
    chips: Vec<Button>,
    search: Input,
    list: HoldBrowser,
    rows: Vec<Row>,
    filter: Filter,
}


impl ResultsWindow {
    // Created hidden: it is shown with View results or Open diff.
    pub(crate) fn new(sender: Sender<Message>) -> Self {
        let mut window = Window::default()
            .with_size(WIDTH, HEIGHT)
            .with_label("Results")
            .center_screen();
        window.set_color(theme::BACKGROUND);
        window.make_resizable(true);

        let chips = FILTERS
            .iter()
            .enumerate()
            .map(|(i, (filter, label))| {
                let x = MARGIN + i as i32 * (CHIP_WIDTH + CHIP_GAP);
                let mut chip = button(x, 12, CHIP_WIDTH, label);
                chip.set_label_size(12);
                chip.emit(sender, Message::Filter(*filter));
                chip
            })
            .collect();

        let mut search = field(MARGIN, SEARCH_TOP, WIDTH - 2 * MARGIN);
        search.set_tooltip("Filter by key, value name or data");
        search.set_trigger(CallbackTrigger::Changed);
        search.emit(sender, Message::Refresh);

        let mut list = HoldBrowser::new(
            MARGIN,
            LIST_TOP,
            WIDTH - 2 * MARGIN,
            HEIGHT - LIST_TOP - MARGIN,
            None
        );
        list.set_frame(theme::OUTLINE_BOX);
        list.set_color(theme::FIELD);
        list.set_text_size(12);
        list.set_column_char('\t');
        list.set_column_widths(&[SYMBOL_WIDTH]);
        list.set_format_char(FORMAT);

        // Only the list grows when the window is resized, and the long texts
        // are wrapped again to the new width.
        window.resizable(&list);
        window.resize_callback(move |_, _, _, _, _| sender.send(Message::Refresh));
        window.end();

        ResultsWindow {
            window,
            chips,
            search,
            list,
            rows: Vec::new(),
            filter: Filter::All,
        }
    }

    pub(crate) fn load(&mut self, result: &DiffResult) {
        let diff = &result.diff;
        let summary = &diff.summary.registry;

        let counts = [
            diff.summary.total,
            summary.keys_added,
            summary.keys_deleted,
            summary.values_added,
            summary.values_deleted,
            summary.values_modified,
        ];

        for (chip, ((_, label), count)) in self.chips.iter_mut().zip(FILTERS.iter().zip(counts)) {
            chip.set_label(&format!("{label} {}", format_count(count)));
        }

        self.window.set_label(&format!("Results · {}", file_name(&result.path)));
        self.rows = rows(diff);
        self.filter = Filter::All;
        self.search.set_value("");
        self.render();
    }

    pub(crate) fn set_filter(&mut self, filter: Filter) {
        self.filter = filter;
        self.render();
    }

    // Fills the list with the rows that pass the filter and the search.
    pub(crate) fn render(&mut self) {
        for (chip, (filter, _)) in self.chips.iter_mut().zip(FILTERS) {
            style_chip(chip, filter == self.filter);
        }

        let query = self.search.value().to_lowercase();
        let fit = Fit::new(self.list.w() - SYMBOL_WIDTH - LIST_PADDING);
        self.list.clear();

        for row in self.rows.iter().filter(|row| row.matches(self.filter, &query)) {
            for line in row.lines(&fit) {
                self.list.add(&line);
            }
        }

        if self.list.size() == 0 {
            self.list.add(&empty());
        }

        self.window.redraw();
    }

    pub(crate) fn show(&mut self) {
        self.window.show();
    }

    pub(crate) fn hide(&mut self) {
        self.window.hide();
        self.list.clear();
        self.rows.clear();
    }
}
