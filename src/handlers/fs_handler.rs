use crossterm::event::KeyCode;
use crate::app::App;
use crate::fs::fs::MoveCurseDirection;

pub fn handle_fs(app: &mut App, key_code: KeyCode) -> bool {
    match key_code {
        KeyCode::Down => {
            app.fs.update_current_on(MoveCurseDirection::Down)
        }
        KeyCode::Up => {
            app.fs.update_current_on(MoveCurseDirection::Up)
        }
        _ => false
    }
}