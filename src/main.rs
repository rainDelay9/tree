use std::fs;
use std::path::PathBuf;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let mut path = PathBuf::new();
    let local = arguments.next().unwrap_or(".".to_owned());
    path.push(&local);
    println!("{}", local);
    print_path(path);
}

fn print_path(path: PathBuf) {
    print_path_helper(path, String::new());
}

fn print_path_helper(path: PathBuf, prefix: String) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                // Here, `entry` is a `DirEntry`.
                if let Ok(md) = entry.metadata() {
                    let file_name = match entry.file_name().into_string() {
                        Ok(s) => s,
                        Err(_) => return,
                    };
                    if file_name.starts_with(".") {
                        continue;
                    }
                    println!("{}", format_path(file_name, prefix.clone()));
                    if md.is_dir() {
                        let new_prefix = prefix.clone() + " ";
                        print_path_helper(entry.path(), new_prefix);
                    } else {
                    }
                    // Now let's show our entry's permissions!
                    //println!("{:?}: {:?}", entry.file_name(), md.is_file());
                }
            }
        }
    }
}

fn format_path(name: String, prefix: String) -> String {
    format!("{}|- {}", prefix, name)
}
