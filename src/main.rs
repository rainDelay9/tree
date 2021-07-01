use core::panic;
use std::fs;
use std::fs::metadata;
use std::path::PathBuf;
use structopt::StructOpt;
use termion::{color, style};

#[derive(Debug, StructOpt)]
#[structopt(name = "tree", about = "show dir tree")]
struct Opt {
    /// Input path
    #[structopt(parse(from_os_str), short = "p", long = "path", default_value = ".")]
    path: PathBuf,

    /// Show hidden files
    #[structopt(long)]
    hidden: bool,
}

fn main() {
    let args = Opt::from_args();
    let path = args.path;
    let md = metadata(&path).expect("non-existent path!");
    if !md.is_dir() {
        panic!("input path is not a directory!");
    }
    println!(
        "{}",
        to_colored(path.to_str().expect("internal error!").to_owned())
    );
    print_path(path, args.hidden);
}

fn print_path(path: PathBuf, show_hidden: bool) {
    print_path_helper(path, show_hidden, String::new());
}

fn print_path_helper(path: PathBuf, show_hidden: bool, prefix: String) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(md) = entry.metadata() {
                    let file_name = match entry.file_name().into_string() {
                        Ok(s) => s,
                        Err(_) => return,
                    };
                    if !show_hidden && file_name.starts_with(".") {
                        continue;
                    }
                    println!("{}", format_path(file_name, prefix.clone(), md.is_file()));
                    if md.is_dir() {
                        let new_prefix = prefix.clone() + "  ";
                        print_path_helper(entry.path(), show_hidden, new_prefix);
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
