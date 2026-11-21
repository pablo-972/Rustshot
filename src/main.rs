mod launch;


use std::env;


fn main() -> anyhow::Result<()> {
    let is_gui = env::args_os().len() == 1 && launch::is_gui();

    if is_gui {
        launch::close_console();
        rustshot_gui::run()
    } else {
        rustshot_cli::run()
    }
}
