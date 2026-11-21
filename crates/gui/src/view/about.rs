use fltk::dialog;


const TEXT: &str = concat!(
    "Rustshot ",
    env!("CARGO_PKG_VERSION"),
    "\n\n",
    "Takes snapshots of the Windows registry and compares them.\n\n",
    "License: GNU Affero General Public License v3."
);


pub(crate) fn about() {
    dialog::message_title("About Rustshot");
    dialog::message_default(TEXT);
}
