use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Padding, Paragraph},
    Frame,
};

use crate::{app::App, constants::{self, FREQUENCIES, Q_FACTOR}, slider::Slider};

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Indexed(8)))
        .padding(Padding::horizontal(1))
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Rust Noise TUI",
        Style::default(),
    ))
    .block(title_block);

    f.render_widget(title, chunks[0]);

    let status_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Indexed(8)))
        .padding(Padding::horizontal(1))
        .style(Style::default());

    let status = Paragraph::new(Text::styled(
        format!("{}Hz \t {}dB gain \t {} Q factor", FREQUENCIES[app.currently_changing], app.vals[app.currently_changing], Q_FACTOR),
        Style::default(),
    ))
    .block(status_block);
    f.render_widget(status, chunks[2]);

    let slider_chunks = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([Constraint::Fill(1);constants::FREQUENCIES.len()])
        .split(chunks[1]);

    for i in 0..constants::FREQUENCIES.len() {
        let slider_block = Block::default()
            .borders(Borders::ALL)
            .padding(Padding::uniform(1))
            .border_style(match app.currently_changing {
                c if c == i => Style::default().fg(Color::Indexed(1)),
                _ => Style::default().fg(Color::Indexed(8)),
            })
            .style(Style::default());

        let slider = Slider::new(app.vals[i], constants::MINIMUM_DB, constants::MAXIMUM_DB).block(slider_block);
    
        f.render_widget(slider, slider_chunks[i]);
    }

}
