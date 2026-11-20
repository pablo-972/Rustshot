use fltk::image::SvgImage;
use fltk::prelude::*;


const FOLDER_OPEN: &str = include_str!("../../icons/folder-open.svg");
const SEARCH: &str = include_str!("../../icons/search.svg");
const RUSTSHOT: &str = include_str!("../../icons/rustshot.svg");
const SIZE: i32 = 16; // Size of the icons, in pixels.
const APP_SIZE: i32 = 48; // Windows scales it down for the title bar.


pub(super) fn folder_open() -> Option<SvgImage> {
    icon(FOLDER_OPEN)
}


pub(super) fn search() -> Option<SvgImage> {
    icon(SEARCH)
}


// Icon of the windows, the same one the .exe has.
pub(super) fn app() -> Option<SvgImage> {
    let mut image = SvgImage::from_data(RUSTSHOT).ok()?;
    image.scale(APP_SIZE, APP_SIZE, true, true);
    Some(image)
}


// None if the SVG cannot be read: the button is then shown without icon.
fn icon(svg: &str) -> Option<SvgImage> {
    let mut image = SvgImage::from_data(svg).ok()?;
    image.scale(SIZE, SIZE, true, true);
    Some(image)
}
