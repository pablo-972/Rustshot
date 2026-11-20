mod launch;

use std::env;


fn main() -> anyhow::Result<()> {
    let is_gui = env::args_os().len() == 1 && launch::is_gui();
    if is_gui { launch::free_console(); gui::run() } else { cli::run() }
}
