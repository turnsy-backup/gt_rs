mod app;
mod ui;
mod util;

use crate::app::App;
use crate::util::add_dir;

use std::path::PathBuf;

use serde_derive::{Deserialize, Serialize};

const DEFAULT_LIST_FILE: &str = "gt_list.txt";
const DEFAULT_GT_PATH_FILE: &str = "gt_path.txt";
const DEFAULT_DIR_DEPTH: i32 = -1;

enum Mode {
    View,
    Delete,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    dir_depth: i32,
    list_path: String,
    gt_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            dir_depth: DEFAULT_DIR_DEPTH,
            list_path: DEFAULT_LIST_FILE.to_string(),
            gt_path: DEFAULT_GT_PATH_FILE.to_string(),
        }
    }
}

fn start(list_file_path: PathBuf, gt_file_path: PathBuf) {
    let mut app = App::new(list_file_path.clone(), gt_file_path.clone());
    app.run().unwrap();
}

fn main() {
    let cfg: Config = confy::load("gt", None).unwrap();

    let cfg_root = confy::get_configuration_file_path("gt", None)
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();

    let list_path = cfg_root.join(&cfg.list_path);
    let gt_path = cfg_root.join(&cfg.gt_path);

    let root_command = std::env::args().nth(1);
    let dir_command = std::env::args().nth(2);

    match root_command.as_deref() {
        Some("add") => add_dir(dir_command.as_deref(), &list_path),
        None => start(list_path, gt_path),
        _ => panic!("Illegal first argument"),
    }
}
