#[allow(unreachable_code)]
#[allow(unused_imports)]

use crate::audio::Song::SongStruct;

use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Widget},
    Terminal,
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

pub fn ui(songs: Vec<SongStruct>) -> Result<(), io::Error> {


    //setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let  song_vec_string:Vec<String> = songs.into_iter().map(|song| song.title).collect();

    
    let song_list: Vec<_> = song_vec_string.into_iter().map(ListItem::new).collect();    


    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let block = Block::default().title("Rust-Play").borders(Borders::ALL);
            let list = List::new(song_list.clone()).block(Block::default().title("list").borders(Borders::ALL)).style(Style::default().fg(Color::Red))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");;
            rect.render_widget(list, size);
        })?;
    }
    Ok(())
}
