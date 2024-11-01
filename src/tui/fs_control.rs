use ratatui::style::Stylize;
use failure::Error;
use ratatui::Frame;
use ratatui::layout::Alignment;
use ratatui::prelude::Rect;
use ratatui::symbols::border;
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Paragraph};
use ratatui::widgets::block::Title;
use crate::app::App;

pub struct FsControllerState {
    pub relative_position: u16,
}
impl FsControllerState {
    pub fn new() -> Self {
        Self {
            relative_position: 1
        }
    }

    pub fn move_relative_position_up(&mut self) {
        if self.relative_position != 1 {
            self.relative_position -= 1;
        }
    }

    pub fn move_relative_position_down(&mut self) {
        self.relative_position += 1;
    }

    pub fn set_relative_position(&mut self, new_position: u16) {
        self.relative_position = new_position;
    }
}

pub fn draw_fs_controller(app: &mut App, frame: &mut Frame, area: Rect) -> Result<(), Error> {
    let title = Title::from("Directory".cyan().bold());
    let block = Block::bordered()
        .title(title.alignment(Alignment::Left))
        .border_set(border::THICK);
    let windows_size = area.height;
    if app.fs_controller_state.relative_position > windows_size {
        app.fs_controller_state.set_relative_position(windows_size);
    }
    let relative_position = app.fs_controller_state.relative_position;
    let idx = app.fs.on_item_idx as u16;
    let mut file_entries = vec![];
    for position in (idx + 1 - relative_position)..(idx + windows_size - relative_position + 1) {
        let path_name = app.fs.path_items.get(position as usize).unwrap().path_name.clone();
        if position == idx {
            file_entries.push(Line::from(vec![path_name.into()]).cyan().underlined().bold());
        } else {
            file_entries.push(Line::from(vec![path_name.into()]).red());
        }
    }
    let file_entry_status = Text::from(file_entries);
    let fs_control_block = Paragraph::new(file_entry_status).block(block);
    frame.render_widget(fs_control_block, area);
    Ok(())
}