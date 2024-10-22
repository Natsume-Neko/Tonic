// use std::thread::{sleep, spawn};
// use std::time::Duration;
use failure::Error;
use crate::app::App;

// mod player;
mod tui;
mod player;
mod handlers;
mod app;
mod fs;

fn main() -> Result<(), Error>{
    let mut app = App::new();
    app.run()
}