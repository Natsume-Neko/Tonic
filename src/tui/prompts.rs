use failure::Error;
use ratatui::style::Stylize;
use ratatui::Frame;
use ratatui::layout::Alignment;
use ratatui::prelude::Rect;
use ratatui::widgets::{Block, Paragraph};
use ratatui::widgets::block::{Title};
use ratatui::symbols::border;
use ratatui::text::{Line, Text};
use crate::app::App;

pub fn draw_prompts(frame: &mut Frame, area: Rect) -> Result<(), Error> {
    let instructions = Text::from(vec![Line::from(vec![
        " Play/Resume <S> Quit<Q> ".into(),
    ])]);
    let title = Title::from("Instructions".bold().red());
    let block = Block::bordered()
        .title(title.alignment(Alignment::Left))
        .border_set(border::THICK);
    let prompts_block = Paragraph::new(instructions)
        .centered()
        .block(block);
    frame.render_widget(prompts_block, area);
    Ok(())
}

pub fn draw_player_status(app: &App, frame: &mut Frame, area: Rect) -> Result<(), Error> {
    let block = Block::bordered()
        .border_set(border::THICK);
    let is_play = if app.music_player.is_play() {
        "⏸︎"
    } else {
        "⏵︎"
    };
    let playing_status = Text::from(vec![
        Line::from(vec![ "Now Playing:".into(), ]),
        Line::from(vec![ is_play.into(), ]),
    ]);
    let player_block = Paragraph::new(playing_status)
        .centered()
        .block(block);
    frame.render_widget(player_block, area);
    Ok(())
}