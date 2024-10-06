use crossterm::event::KeyCode;
use crate::app::App;

pub fn handle_player(app: &mut App, key_code: KeyCode) -> bool {
    match key_code {
        KeyCode::Char('s') | KeyCode::Char('S') => {
            // println!("Yes!");
            if app.music_player.is_play() {
                app.music_player.pause()
            } else {
                app.music_player.play()
            }
        }
        _ => false
    }
}