use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, List, ListDirection, ListState},
    Frame,
};

pub fn draw_view(frame: &mut Frame, dirs: &Vec<String>, state: &mut ListState) {
    let list = List::new(dirs.to_vec())
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .style(Style::new().fg(Color::Gray)),
        )
        .highlight_symbol("> ")
        .highlight_style(Style::new().fg(Color::White))
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),                        // This takes up all remaining space
            Constraint::Length(dirs.len() as u16 + 2), // This is your list height
        ])
        .split(frame.area());

    frame.render_stateful_widget(list, chunks[1], state);
}

pub fn draw_delete(frame: &mut Frame, dir: &String) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(frame.area());

    let message = format!("Are you sure you want to delete {} ? (y/n)", dir);

    frame.render_widget(message, chunks[1]);
}
