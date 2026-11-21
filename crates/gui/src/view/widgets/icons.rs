use fltk::image::SvgImage;
use fltk::prelude::*;


const FOLDER_OPEN: &str = include_str!("../../../icons/folder-open.svg");
const SEARCH: &str = include_str!("../../../icons/search.svg");
const RUSTSHOT: &str = include_str!("../../../icons/rustshot.svg");

// Sizes in pixels. Windows scales the icon of the app down for the title bar.
const BUTTON_SIZE: i32 = 16;
const APP_SIZE: i32 = 48;


pub(super) fn folder_open() -> Option<SvgImage> {
    icon(FOLDER_OPEN, BUTTON_SIZE)
}


pub(super) fn search() -> Option<SvgImage> {
    icon(SEARCH, BUTTON_SIZE)
}


// Icon of the windows, the same one the .exe has.
pub(crate) fn app_icon() -> Option<SvgImage> {
    icon(RUSTSHOT, APP_SIZE)
}


// None if the SVG cannot be read: the widget is then shown without icon.
fn icon(svg: &str, size: i32) -> Option<SvgImage> {
    let mut image = SvgImage::from_data(svg).ok()?;
    image.scale(size, size, true, true);
    Some(image)
}
