use std::io;

use anyhow::Result;
use event::Events;
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, Row, Table, TableState},
    Terminal,
};

mod util;
pub struct StatefulTable<'a> {
    state: TableState,
    items: Vec<Vec<&'a str>>,
}

impl<'a> StatefulTable<'a> {
    fn new() -> Self {
        Self {
            state: TableState::default(),
            items: vec![
                vec!["Row11", "Row12", "Row13"],
                vec!["Row21", "Row22", "Row23"],
                vec!["Row31", "Row32", "Row33"],
                vec!["Row41", "Row42", "Row43"],
                vec!["Row51", "Row52", "Row53"],
                vec!["Row61", "Row62", "Row63"],
                vec!["Row71", "Row72", "Row73"],
                vec!["Row81", "Row82", "Row83"],
                vec!["Row91", "Row92", "Row93"],
                vec!["Row101", "Row102", "Row103"],
                vec!["Row111", "Row112", "Row113"],
                vec!["Row121", "Row122", "Row123"],
                vec!["Row131", "Row132", "Row133"],
                vec!["Row141", "Row142", "Row143"],
                vec!["Row151", "Row152", "Row153"],
                vec!["Row161", "Row162", "Row163"],
                vec!["Row171", "Row172", "Row173"],
                vec!["Row181", "Row182", "Row183"],
                vec!["Row191", "Row192", "Row193"],
            ],
        }
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
use termion::{event::Key, raw::IntoRawMode, screen::AlternateScreen};
use util::event::{self, Event};
fn main() -> Result<()> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let events = Events::new();
    let mut table = StatefulTable::new();
    loop {
        terminal.draw(|f| {
            // 定制化大小
            // let rects = Layout::default()
            //     .constraints([Constraint::Percentage(100)])
            //     .margin(1)
            //     .split(f.size());
            let seleted_style = Style::default().add_modifier(Modifier::REVERSED | Modifier::BOLD);
            let normal_style = Style::default().fg(tui::style::Color::Red);
            let header = ["Header1", "Header2", "Header3"];
            let rows = table
                .items
                .iter()
                .map(|i| Row::StyledData(i.iter(), normal_style));
            let t = Table::new(header.iter(), rows)
                .block(Block::default().borders(Borders::ALL).title("Junknet"))
                .highlight_style(seleted_style)
                .widths(&[
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                ]);
            f.render_stateful_widget(t, f.size(), &mut table.state);
        })?;
        if let Event::Input(key) = events.next()? {
            match key {
                Key::Char('q') => {
                    break;
                }
                Key::Down => {
                    table.next();
                }
                Key::Up => {
                    table.previous();
                }
                _ => {}
            }
        };
    }
    Ok(())
}
