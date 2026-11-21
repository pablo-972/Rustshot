use fltk::enums::Color;
use fltk::image::SvgImage;
use fltk::prelude::*;


const FOLDER_OPEN: &str = include_str!("../../../icons/folder-open.svg");
const SEARCH: &str = include_str!("../../../icons/search.svg");
const RUSTSHOT: &str = include_str!("../../../icons/rustshot.svg");

// Sizes in pixels. Windows scales the icon of the app down for the title bar.
const BUTTON_SIZE: i32 = 16;
const APP_SIZE: i32 = 48;


pub(super) fn folder_open(color: Color) -> Option<SvgImage> {
    icon(&paint(FOLDER_OPEN, color), BUTTON_SIZE)
}


pub(super) fn search(color: Color) -> Option<SvgImage> {
    icon(&paint(SEARCH, color), BUTTON_SIZE)
}


// Icon of the windows, the same one the .exe has. It has its own colors.
pub(crate) fn app_icon() -> Option<SvgImage> {
    icon(RUSTSHOT, APP_SIZE)
}


// Draws an icon in the middle of a widget. The icons are drawn each time
// instead of set as the image of the widget, so they take the colors of the
// theme in use.
pub(super) fn draw_centered<W: WidgetExt>(image: Option<SvgImage>, widget: &W) {
    if let Some(mut image) = image {
        let x = widget.x() + (widget.w() - image.w()) / 2;
        let y = widget.y() + (widget.h() - image.h()) / 2;
        image.draw(x, y, image.w(), image.h());
    }
}


// The icons are drawn with currentColor: it is replaced by a color of the
// palette, as #rrggbb.
fn paint(svg: &str, color: Color) -> String {
    let (r, g, b) = color.to_rgb();
    svg.replace("currentColor", &format!("#{r:02x}{g:02x}{b:02x}"))
}


// None if the SVG cannot be read: the widget is then shown without icon.
fn icon(svg: &str, size: i32) -> Option<SvgImage> {
    let mut image = SvgImage::from_data(svg).ok()?;
    image.scale(size, size, true, true);
    Some(image)
}
