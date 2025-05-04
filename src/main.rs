mod app;
mod ui;
mod util;

use crate::app::App;
use crate::util::add_dir;

use std::path::PathBuf;

use serde_derive::{Deserialize, Serialize};

const DEFAULT_LIST_FILE: &str = "gt_list.txt";
const DEFAULT_DIR_DEPTH: i32 = -1;

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
            dir_depth: DEFAULT_DIR_DEPTH,
            list_path: DEFAULT_LIST_FILE.to_string(),
        }
    }
}

fn start(path: PathBuf) {
    let mut app = App::new(path);
    if let Ok(selected_dir) = app.run() {
        println!("{}", selected_dir);
    }
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
