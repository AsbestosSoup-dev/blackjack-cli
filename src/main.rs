mod models;
mod ui;
mod app;

use std::io;

use crate::app::App;

// struct PlayerQueue {
//     waitlist: Vec<Player>,
// }

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
