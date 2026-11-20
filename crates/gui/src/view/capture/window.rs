use fltk::app::{self, Sender};
use fltk::button::Button;
use fltk::enums::{Cursor, FrameType};
use fltk::frame::Frame;
use fltk::input::Input;
use fltk::prelude::*;
use fltk::window::Window;

use super::layout::{
    BUTTON_WIDTH,
    BUTTON_X,
    FOOTER_TOP,
    HEIGHT,
    MARGIN,
    ROW_HEIGHT,
    ROW_TOP,
    WIDTH,
};
use super::step_row::{StepRow, StepView};
use crate::theme;
use crate::types::{
    DiffResult,
    Message,
    Shot,
    State,
    Step,
    Task,
};
use crate::view::format::{file_name, format_count};
use crate::view::widgets::{
    button,
    caption,
    field,
    line,
    separator,
    set_enabled,
    style_button,
};


pub(crate) struct CaptureWindow {
    window: Window,
    output: Input,
    browse: Button,
    first: StepRow,
    second: StepRow,
    diff: StepRow,
    results: Button,
    status: Frame,
    open: Button,
    clear: Button,
}


impl CaptureWindow {
    pub(crate) fn new(sender: Sender<Message>, output: &str) -> Self {
        let mut window = Window::default()
            .with_size(WIDTH, HEIGHT)
            .with_label("Rustshot")
            .center_screen();
        window.set_color(theme::BACKGROUND);

        // Closing this window closes the results window too.
        window.set_callback(|_| app::quit());

        caption(MARGIN, 14, "Output folder");
        let mut output_field = field(MARGIN, 34, BUTTON_X - MARGIN - 10);
        output_field.set_value(output);

        let mut browse = button(BUTTON_X, 34, BUTTON_WIDTH, "Browse…");
        browse.emit(sender, Message::Browse);

        let mut first = StepRow::new(
            0,
            "First shot name",
            "before.json",
            "Clean system, before running the sample",
        );
        first.button.emit(sender, Message::Shoot(Step::First));

        let mut second = StepRow::new(
            1,
            "Second shot name",
            "after.json",
            "Take the first shot to enable this step",
        );
        second.button.emit(sender, Message::Shoot(Step::Second));

        let mut diff = StepRow::new(
            2,
            "Diff name",
            "diff.json",
            "Needs both shots"
        );
        diff.button.emit(sender, Message::Compare);

        let results_top = ROW_TOP + 2 * ROW_HEIGHT + 66;
        let mut results = button(BUTTON_X, results_top, BUTTON_WIDTH, "View results");
        results.emit(sender, Message::ViewResults);

        let (status, open, clear) = footer(sender);

        window.end();
        window.show();

        CaptureWindow {
            window,
            output: output_field,
            browse,
            first,
            second,
            diff,
            results,
            status,
            open,
            clear,
        }
    }

    pub(crate) fn output(&self) -> String {
        self.output.value().trim().to_string()
    }

    pub(crate) fn set_output(&mut self, path: &str) {
        self.output.set_value(path);
    }

    pub(crate) fn shot_name(&self, step: Step) -> String {
        match step {
            Step::First => self.first.name(),
            Step::Second => self.second.name(),
        }
    }

    pub(crate) fn diff_name(&self) -> String {
        self.diff.name()
    }

    // The three file names, to check that they are different.
    pub(crate) fn names(&self) -> [String; 3] {
        [self.first.name(), self.second.name(), self.diff.name()]
    }

    pub(crate) fn render(&mut self, state: &State) {
        let idle = state.busy.is_none();
        let first_done = state.first.is_some();
        let both_done = first_done && state.second.is_some();
        let has_diff = state.diff.is_some();

        set_enabled(&mut self.output, idle);
        set_enabled(&mut self.browse, idle);
        set_enabled(&mut self.open, idle);
        set_enabled(&mut self.clear, idle);

        self.first.render(StepView {
            enabled: idle,
            primary: !first_done,
            running: state.busy == Some(Task::Shoot(Step::First)),
            label: if first_done { "Retake 1st shot" } else { "Take 1st shot" },
            done: state.first.as_ref().map(shot_info),
        });

        self.second.hint = if first_done {
            "Run the sample, then take this shot"
        } else {
            "Take the first shot to enable this step"
        };

        self.second.render(StepView {
            enabled: idle && first_done,
            primary: first_done && state.second.is_none(),
            running: state.busy == Some(Task::Shoot(Step::Second)),
            label: if state.second.is_some() { "Retake 2nd shot" } else { "Take 2nd shot" },
            done: state.second.as_ref().map(shot_info),
        });

        self.diff.hint = if both_done { "Ready to compare" } else { "Needs both shots" };

        self.diff.render(StepView {
            enabled: idle && both_done,
            primary: both_done && !has_diff,
            running: matches!(state.busy, Some(Task::Compare | Task::OpenDiff)),
            label: "Compare",
            done: state.diff.as_ref().map(diff_info),
        });

        set_enabled(&mut self.results, idle && has_diff);
        style_button(&mut self.results, idle && has_diff);

        self.status.set_label(&state.status.text);
        self.status.set_label_color(
            if state.status.error { theme::DANGER } else { theme::TEXT_SECONDARY }
        );

        let cursor = if idle { Cursor::Default } else { Cursor::Wait };
        self.window.set_cursor(cursor);
        self.window.redraw();
    }
}


// Bottom bar: status text, Open diff and Clear.
fn footer(sender: Sender<Message>) -> (Frame, Button, Button) {
    let mut background = Frame::new(0, FOOTER_TOP, WIDTH, HEIGHT - FOOTER_TOP, None);
    background.set_frame(FrameType::FlatBox);
    background.set_color(theme::FOOTER);
    separator(0, FOOTER_TOP, WIDTH);

    let status = line(MARGIN, FOOTER_TOP + 8, 240, 30);

    let mut open = button(
        BUTTON_X - BUTTON_WIDTH - 10,
        FOOTER_TOP + 8,
        BUTTON_WIDTH,
        "Open diff…"
    );
    open.emit(sender, Message::OpenDiff);

    let mut clear = button(BUTTON_X, FOOTER_TOP + 8, BUTTON_WIDTH, "Clear");
    clear.emit(sender, Message::Clear);

    (status, open, clear)
}


fn shot_info(shot: &Shot) -> String {
    let summary = &shot.snapshot.summary;

    format!(
        "{} keys, {} values · {}",
        format_count(summary.registry_keys),
        format_count(summary.registry_values),
        file_name(&shot.path)
    )
}


fn diff_info(result: &DiffResult) -> String {
    format!(
        "{} changes · {}",
        format_count(result.diff.summary.total),
        file_name(&result.path)
    )
}
