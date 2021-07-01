use core::panic;
use std::fs;
use std::fs::metadata;
use std::path::PathBuf;
use termion::{color, style};

fn main() {
    let mut arguments = std::env::args().skip(1);
    let mut path = PathBuf::new();
    let local = arguments.next().unwrap_or(".".to_owned());
    path.push(&local);
    let md = metadata(&path).expect("non-existent path!");
    if !md.is_dir() {
        panic!("input path is not a directory!");
    }
    println!("{}", to_colored(local));
    print_path(path);
}

fn print_path(path: PathBuf) {
    print_path_helper(path, String::new());
}

fn print_path_helper(path: PathBuf, prefix: String) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(md) = entry.metadata() {
                    let file_name = match entry.file_name().into_string() {
                        Ok(s) => s,
                        Err(_) => return,
                    };
                    if file_name.starts_with(".") {
                        continue;
                    }
                    println!("{}", format_path(file_name, prefix.clone(), md.is_file()));
                    if md.is_dir() {
                        let new_prefix = prefix.clone() + " ";
                        print_path_helper(entry.path(), new_prefix);
                    }
                }
            }
        }
    }
}

fn format_path(name: String, prefix: String, is_file: bool) -> String {
    if is_file {
        format!("{}|- {}", prefix, name)
    } else {
        format!("{}|- {}", prefix, to_colored(name))
    }
}

fn to_colored(s: String) -> String {
    format!("{}{}{}", color::Fg(color::Cyan), s, style::Reset)
}
