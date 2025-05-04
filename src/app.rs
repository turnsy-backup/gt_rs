use crate::ui::{draw_delete, draw_view};
use crate::util::remove_dir;
use crate::Mode;
use std::fs::{exists, read_to_string, File};
use std::io::Result;
use std::path::PathBuf;

use crossterm::event::{self, Event, KeyCode};
use ratatui::widgets::ListState;
use ratatui::Frame;

pub struct App {
    dirs: Vec<String>,
    list_state: ListState,
    mode: Mode,
    list_path: PathBuf,
}

impl App {
    pub fn new(list_path: PathBuf) -> Self {
        if !exists(&list_path).unwrap() {
            let _ = File::create(&list_path);
        }

        let dirs: Vec<String> = read_to_string(&list_path)
            .unwrap()
            .lines()
            .map(String::from)
            .collect();

        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            dirs,
            list_state,
            mode: Mode::View,
            list_path,
        }
    }

    pub fn run(&mut self) -> Result<String> {
        let mut terminal = ratatui::init();

        loop {
            terminal.draw(|frame| self.render(frame))?;

            if let Event::Key(key) = event::read()? {
                if self.handle_input(key.code) {
                    break;
                }
            }
        }

        ratatui::restore();
        Ok(self.get_selected_dir())
    }

    fn render(&mut self, frame: &mut Frame) {
        match self.mode {
            Mode::View => draw_view(frame, &self.dirs, &mut self.list_state),
            Mode::Delete => draw_delete(frame, &self.dirs[self.list_state.selected().unwrap()]),
        }
    }

    fn handle_input(&mut self, key: KeyCode) -> bool {
        match self.mode {
            Mode::View => self.handle_view_input(key),
            Mode::Delete => self.handle_delete_input(key),
        }
    }

    fn handle_view_input(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Down => {
                self.list_state.select_next();
                false
            }
            KeyCode::Up => {
                self.list_state.select_previous();
                false
            }
            KeyCode::Char(c) => {
                if c.is_ascii_digit() {
                    let new_index = c.to_digit(10).unwrap() as usize - 1;
                    self.list_state.select(Some(new_index));
                    false
                } else {
                    match c {
                        'd' | 'D' => {
                            self.mode = Mode::Delete;
                            false
                        }
                        'q' | 'Q' => true,
                        _ => false,
                    }
                }
            }
            _ => true,
        }
    }

    fn handle_delete_input(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Char(c) => match c {
                'y' => {
                    remove_dir(
                        self.list_state.selected().unwrap(),
                        &mut self.dirs,
                        &self.list_path,
                    );
                    self.mode = Mode::View;
                    false
                }
                'n' => {
                    self.mode = Mode::View;
                    false
                }
                'q' => true,
                _ => false,
            },
            _ => true,
        }
    }

    fn get_selected_dir(&self) -> String {
        self.dirs[self.list_state.selected().unwrap()].clone()
    }
}
