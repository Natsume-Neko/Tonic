use crossterm::event::KeyCode;
use crate::app::App;
use crate::fs::fs::MoveCurseDirection;

pub fn handle_fs(app: &mut App, key_code: KeyCode) -> bool {
    match key_code {
        KeyCode::Down => {
            app.fs_controller_state.move_relative_position_down();
            app.fs.update_current_on(MoveCurseDirection::Down)
        }
        KeyCode::Up => {
            app.fs_controller_state.move_relative_position_up();
            app.fs.update_current_on(MoveCurseDirection::Up)
        }
        KeyCode::Enter => {
            if app.fs.get_to_curse_on() {
                app.fs_controller_state.set_relative_position(1);
                return true
            } else {
                app.music_player.add_to_playlist(app.fs.on_item_dir.to_string_lossy().to_string(), true);
            }
            false
        }
        _ => false
    }
}