use std::sync::mpsc;
use std::thread::{sleep, spawn};
use std::time::Duration;
use crate::player::player::MusicPlayer;
use crossterm::event::{self, KeyCode, Event, KeyEventKind};
use ratatui::Frame;
use ratatui::layout::Direction;
use ratatui::prelude::{Constraint, Layout};
use crate::fs;
use crate::handlers::fs_handler::handle_fs;
use crate::handlers::player_handler::handle_player;
use crate::tui::fs_control::{draw_fs_controller, FsControllerState};
use crate::tui::prompts::{draw_player_status, draw_prompts};

pub struct App {
    pub music_player: MusicPlayer,
    pub fs: fs::fs::FileSystem,
    pub fs_controller_state: FsControllerState,
}

impl App {
    pub fn new() -> Self {
        Self {
            music_player: MusicPlayer::new(),
            fs: fs::fs::FileSystem::new(),
            fs_controller_state: FsControllerState::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), failure::Error> {
        // let f_name = String::from("Assets/Example/wowaka - アンハッピーリフレイン.mp3");
        // self.music_player.add_to_playlist(f_name, true);

        let mut terminal = ratatui::init();

        let (tx, rx) = mpsc::channel();

        let event_sender = spawn(move || {
            sleep(Duration::from_millis(15));
             loop {
                 let event_result = event::read();
                 match event_result {
                     Ok(evt) => {
                         if let Event::Key(key_event) = evt {
                             if key_event.kind != KeyEventKind::Press {
                                 continue;
                             }
                             match tx.send(key_event.code.clone()) {
                                 Ok(_) => {}
                                 _ => { return; }
                             }
                             match key_event.code {
                                 KeyCode::Char('Q') | KeyCode::Char('q') => {
                                     return;
                                 }
                                 _ => {}
                             }
                         }
                     }
                     _ => return
                 }
             }
        });
        loop {
            sleep(Duration::from_millis(10));

            terminal.draw(|frame| self.draw_tui(frame))?;

            if let Ok(code) = rx.recv() {
                match code {
                    KeyCode::Char('Q') | KeyCode::Char('q') => {
                        break;
                    }
                    _ => {
                        handle_player(self, code);
                        handle_fs(self, code);
                    }
                }
            }
        }
        event_sender.join().unwrap();
        ratatui::restore();
        Ok(())
    }
    fn draw_tui(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(frame.area());
        let layout_upper = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(layout[0]);
        draw_prompts(frame, layout_upper[0]).unwrap();
        draw_fs_controller(self, frame, layout_upper[1]).unwrap();
        draw_player_status(self, frame, layout[1]).unwrap();
    }
}