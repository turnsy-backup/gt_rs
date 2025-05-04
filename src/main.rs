use std::path::PathBuf;

use crossterm::event::{self, Event, KeyCode};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::widgets::{Block, BorderType, Borders, List, ListDirection, ListState};
use ratatui::Frame;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::env::current_dir;
use std::fs::exists;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;

enum Mode {
    View,
    Delete,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    dir_depth: i32,
    list_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            dir_depth: 1,
            list_path: "dc_list.txt".to_string(),
        }
    }
}

fn get_or_create_list_file(list_path: &PathBuf) -> File {
    File::options()
        .append(true)
        .create(true)
        .write(true)
        .open(list_path)
        .unwrap()
}

fn overwrite_list_file(list_path: &PathBuf, dirs: &Vec<String>) {
    let mut list_file = File::create(list_path).unwrap();
    for path in dirs {
        writeln!(&mut list_file, "{path}").unwrap();
    }
}

fn add_dir(path_arg: Option<&str>, list_path: &PathBuf) {
    let path: String = match path_arg {
        Some(path) => path.to_string(),
        None => current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap(),
    };

    let mut list_file = get_or_create_list_file(list_path);
    writeln!(&mut list_file, "{path}").unwrap();
}

fn remove_dir(index: usize, dirs: &mut Vec<String>, list_path: &PathBuf) {
    dirs.remove(index);
    overwrite_list_file(list_path, dirs);
}

fn start(path: PathBuf) {
    // Create list file if it doesn't exist
    if !exists(&path).unwrap() {
        let _ = File::create(&path);
    }

    // list file contents, for now
    let mut dirs: Vec<String> = read_to_string(&path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut mode = Mode::View;
    let mut terminal = ratatui::init();
    let mut list_state = ListState::default();
    list_state.select(Some(0));
    loop {
        match mode {
            Mode::View => {
                terminal
                    .draw(|frame| draw_view(frame, &dirs, &mut list_state))
                    .unwrap();
                if let Event::Key(key) = event::read().unwrap() {
                    match key.code {
                        KeyCode::Down => list_state.select_next(),
                        KeyCode::Up => list_state.select_previous(),
                        KeyCode::Char(c) => {
                            if c.is_ascii_digit() {
                                let new_index = c.to_digit(10).unwrap() as usize - 1;
                                list_state.select(Some(new_index));
                            }

                            if c == 'D' || c == 'd' {
                                mode = Mode::Delete;
                            }

                            if c == 'q' || c == 'Q' {
                                break;
                            }
                        }
                        _ => break,
                    }
                }
            }
            Mode::Delete => {
                terminal
                    .draw(|frame| draw_delete(frame, &dirs[list_state.selected().unwrap()]))
                    .unwrap();
                if let Event::Key(key) = event::read().unwrap() {
                    match key.code {
                        KeyCode::Char(c) => match c {
                            'y' => {
                                remove_dir(list_state.selected().unwrap(), &mut dirs, &path);
                                mode = Mode::View;
                            }
                            'n' => mode = Mode::View,
                            'q' => break,
                            _ => {}
                        },
                        _ => break,
                    }
                }
            }
        };
    }
    ratatui::restore();

    println!("{}", dirs[list_state.selected().unwrap()]);
}

fn draw_view(frame: &mut Frame, dirs: &Vec<String>, state: &mut ListState) {
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

fn draw_delete(frame: &mut Frame, dir: &String) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(frame.area());

    let message = format!("Are you sure you want to delete {} ? (y/n)", dir);

    frame.render_widget(message, chunks[1]);
}
fn main() {
    let cfg: Config = confy::load("dc", None).unwrap();
    let mut list_path: PathBuf = confy::get_configuration_file_path("dc", None).unwrap();
    list_path.push(cfg.list_path);

    let root_command = std::env::args().nth(1);
    let dir_command = std::env::args().nth(2);

    match root_command.as_deref() {
        Some("add") => add_dir(dir_command.as_deref(), &list_path),
        None => start(list_path),
        _ => panic!("Illegal first argument"),
    }
}
