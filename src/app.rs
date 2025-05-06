use crate::ui::{draw_delete, draw_view};
use crate::util::{get_or_create_file, remove_dir};
use crate::Mode;
use std::fs::{exists, read_to_string, File};
use std::io::{Result, SeekFrom, Write};
use std::path::PathBuf;

use crossterm::event::{self, Event, KeyCode};
use ratatui::widgets::ListState;
use ratatui::Frame;

pub struct App {
    dirs: Vec<String>,
    list_state: ListState,
    mode: Mode,
    list_path: PathBuf,
    gt_path: PathBuf,
}

enum ExitReason {
    Quit,
    GoTo,
    Interact,
    Delete,
    DeleteExit,
}

impl App {
    pub fn new(list_path: PathBuf, gt_path: PathBuf) -> Self {
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
            gt_path,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut terminal = ratatui::init();

        loop {
            terminal.draw(|frame| self.render(frame))?;

            if let Event::Key(key) = event::read()? {
                match self.handle_input(key.code) {
                    ExitReason::GoTo => {
                        // write to file
                        let mut gt_file = get_or_create_file(&self.gt_path);
                        gt_file.set_len(0).unwrap();
                        gt_file
                            .write_all(&self.get_selected_dir().as_bytes())
                            .unwrap();
                        break;
                    }
                    ExitReason::Quit => {
                        // clear file
                        let gt_file = get_or_create_file(&self.gt_path);
                        gt_file.set_len(0).unwrap();

                        break;
                    }
                    ExitReason::Delete => {
                        self.mode = Mode::Delete;
                    }
                    ExitReason::DeleteExit => {
                        self.mode = Mode::View;
                    }
                    ExitReason::Interact => {}
                }
            }
        }

        ratatui::restore();
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        match self.mode {
            Mode::View => draw_view(frame, &self.get_dirs_with_prefix(), &mut self.list_state),
            Mode::Delete => draw_delete(frame, &self.dirs[self.list_state.selected().unwrap()]),
        }
    }

    fn handle_input(&mut self, key: KeyCode) -> ExitReason {
        match self.mode {
            Mode::View => self.handle_view_input(key),
            Mode::Delete => self.handle_delete_input(key),
        }
    }

    fn handle_view_input(&mut self, key: KeyCode) -> ExitReason {
        match key {
            KeyCode::Down => {
                self.list_state.select_next();
                ExitReason::Interact
            }
            KeyCode::Up => {
                self.list_state.select_previous();
                ExitReason::Interact
            }
            KeyCode::Char(c) => {
                if c.is_ascii_digit() {
                    let new_index = c.to_digit(10).unwrap() as usize - 1;
                    self.list_state.select(Some(new_index));
                    ExitReason::Interact
                } else {
                    match c {
                        'd' | 'D' => ExitReason::Delete,
                        'q' | 'Q' => ExitReason::Quit,
                        _ => ExitReason::Interact,
                    }
                }
            }
            _ => ExitReason::GoTo,
        }
    }

    fn handle_delete_input(&mut self, key: KeyCode) -> ExitReason {
        match key {
            KeyCode::Char(c) => match c {
                'y' => {
                    remove_dir(
                        self.list_state.selected().unwrap(),
                        &mut self.dirs,
                        &self.list_path,
                    );
                    ExitReason::DeleteExit
                }
                'n' => {
                    self.mode = Mode::View;
                    ExitReason::DeleteExit
                }
                'q' => ExitReason::Quit,
                _ => ExitReason::Interact,
            },
            _ => ExitReason::Interact,
        }
    }

    fn get_selected_dir(&self) -> String {
        self.dirs[self.list_state.selected().unwrap()].clone()
    }

    fn get_dirs_with_prefix(&self) -> Vec<String> {
        self.dirs
            .iter()
            .enumerate()
            .map(|(idx, dir)| format!("[{}] {}", (idx + 1), dir))
            .collect()
    }
}
