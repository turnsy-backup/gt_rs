use std::path::PathBuf;

use dialoguer::Select;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::env::current_dir;
use std::fs::exists;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;

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

fn add_dir(path_arg: Option<&str>, list_path: PathBuf) {
    let path: String = match path_arg {
        Some(path) => path.to_string(),
        None => current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap(),
    };

    let mut list_file = File::options()
        .append(true)
        .write(true)
        .open(list_path)
        .unwrap();

    writeln!(&mut list_file, "{path}").unwrap();
}

fn start(path: PathBuf) {
    // Create list file if it doesn't exist
    if !exists(&path).unwrap() {
        let _ = File::create(&path);
    }

    // list file contents, for now
    let dirs: Vec<String> = read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let selection = Select::new().items(&dirs).interact().unwrap();

    println!("{}", dirs[selection]);
}

fn main() {
    let cfg: Config = confy::load("dc", None).unwrap();
    let mut list_path: PathBuf = confy::get_configuration_file_path("dc", None).unwrap();
    list_path.push(cfg.list_path);

    let root_command = std::env::args().nth(1);
    let dir_command = std::env::args().nth(2);

    match root_command.as_deref() {
        Some("add") => add_dir(dir_command.as_deref(), list_path),
        None => start(list_path),
        _ => panic!("Illegal first argument"),
    }
}
