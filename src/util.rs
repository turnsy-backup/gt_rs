use std::{env::current_dir, fs::File, io::Write, path::PathBuf};

pub fn get_or_create_file(list_path: &PathBuf) -> File {
    File::options()
        .append(true)
        .create(true)
        .write(true)
        .open(list_path)
        .unwrap()
}

pub fn overwrite_list_file(list_path: &PathBuf, dirs: &Vec<String>) {
    let mut list_file = File::create(list_path).unwrap();
    for path in dirs {
        writeln!(&mut list_file, "{path}").unwrap();
    }
}

pub fn add_dir(path_arg: Option<&str>, list_path: &PathBuf) {
    let path: String = match path_arg {
        Some(path) => path.to_string(),
        None => current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap(),
    };

    let mut list_file = get_or_create_file(list_path);
    writeln!(&mut list_file, "{path}").unwrap();
    println!("Added '{}' to gt", path);
}

pub fn remove_dir(index: usize, dirs: &mut Vec<String>, list_path: &PathBuf) {
    dirs.remove(index);
    overwrite_list_file(list_path, dirs);
}
