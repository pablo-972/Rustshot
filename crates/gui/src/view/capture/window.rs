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
    FOOTER_PADDING,
    FOOTER_TOP,
    HEIGHT,
    MARGIN,
    OUTPUT_CAPTION_TOP,
    OUTPUT_TOP,
    OUTPUT_WIDTH,
    RESULTS_TOP,
    SEPARATOR_TOP,
    WIDTH,
};
use super::step_row::{StepRow, StepView};
use crate::format::{file_name, format_count};
use crate::theme;
use crate::types::{
    DiffFile,
    Message,
    ShotFile,
    State,
    Step,
    Task,
};
use crate::view::widgets::{
    app_icon,
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
    clear: Button,
}


impl CaptureWindow {
    pub(crate) fn new(sender: Sender<Message>, output: &str) -> Self {
        let mut window = Window::default()
            .with_size(WIDTH, HEIGHT)
            .with_label("Rustshot")
            .center_screen();
        window.set_color(theme::BACKGROUND);
        window.set_icon(app_icon());

        // Closing this window closes the results window too.
        window.set_callback(|_| app::quit());

        caption(MARGIN, OUTPUT_CAPTION_TOP, "Output folder");
        let mut output_field = field(MARGIN, OUTPUT_TOP, OUTPUT_WIDTH);
        output_field.set_value(output);

        let mut browse = button(BUTTON_X, OUTPUT_TOP, BUTTON_WIDTH, "Browse…");
        browse.emit(sender, Message::Browse);

        // Line between the output folder and the steps.
        separator(MARGIN, SEPARATOR_TOP, WIDTH - 2 * MARGIN);

        let mut first = StepRow::new(
            0,
            "First shot name",
            "before.json",
            sender,
            Message::OpenShot(Step::First)
        );
        first.button.emit(sender, Message::Shoot(Step::First));

        let mut second = StepRow::new(
            1,
            "Second shot name",
            "after.json",
            sender,
            Message::OpenShot(Step::Second)
        );
        second.button.emit(sender, Message::Shoot(Step::Second));

        let mut diff = StepRow::new(
            2,
            "Diff name",
            "diff.json",
            sender,
            Message::OpenDiff
        );
        diff.button.emit(sender, Message::Compare);

        let mut results = button(BUTTON_X, RESULTS_TOP, BUTTON_WIDTH, "View results");
        results.emit(sender, Message::ViewResults);

        let (status, clear) = footer(sender);

        window.end();
        window.show();
        theme::dark_title_bar(window.raw_handle());

        CaptureWindow {
            window,
            output: output_field,
            browse,
            first,
            second,
            diff,
            results,
            status,
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

    // Shows the name of a file opened instead of taken.
    pub(crate) fn set_shot_name(&mut self, step: Step, name: &str) {
        match step {
            Step::First => self.first.set_name(name),
            Step::Second => self.second.set_name(name),
        }
    }

    pub(crate) fn set_diff_name(&mut self, name: &str) {
        self.diff.set_name(name);
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
        set_enabled(&mut self.clear, idle);

        self.first.render(StepView {
            enabled: idle,
            openable: idle,
            primary: !first_done,
            running: running(state, Step::First),
            label: if first_done { "Retake 1st shot" } else { "Take 1st shot" },
            done: state.first.as_ref().map(shot_info),
        });

        self.second.render(StepView {
            enabled: idle && first_done,
            openable: idle && first_done,
            primary: first_done && state.second.is_none(),
            running: running(state, Step::Second),
            label: if state.second.is_some() { "Retake 2nd shot" } else { "Take 2nd shot" },
            done: state.second.as_ref().map(shot_info),
        });

        self.diff.render(StepView {
            enabled: idle && both_done,
            openable: idle,
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


// Bottom bar: status text and Clear.
fn footer(sender: Sender<Message>) -> (Frame, Button) {
    let mut background = Frame::new(0, FOOTER_TOP, WIDTH, HEIGHT - FOOTER_TOP, None);
    background.set_frame(FrameType::FlatBox);
    background.set_color(theme::FOOTER);

    separator(0, FOOTER_TOP, WIDTH);
    let top = FOOTER_TOP + FOOTER_PADDING;
    let status = line(MARGIN, top, OUTPUT_WIDTH, 30);

    let mut clear = button(BUTTON_X, top, BUTTON_WIDTH, "Clear");
    clear.emit(sender, Message::Clear);

    (status, clear)
}


// A shot is running while it is taken or opened.
fn running(state: &State, step: Step) -> bool {
    state.busy == Some(Task::Shoot(step)) || state.busy == Some(Task::OpenShot(step))
}


fn shot_info(shot: &ShotFile) -> String {
    let summary = &shot.snapshot.summary;

    format!(
        "{} keys, {} values · {}",
        format_count(summary.registry_keys),
        format_count(summary.registry_values),
        file_name(&shot.path)
    )
}


fn diff_info(result: &DiffFile) -> String {
    format!(
        "{} changes · {}",
        format_count(result.diff.summary.total),
        file_name(&result.path)
    )
}
