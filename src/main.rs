use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <search_term>", args[0]);
        return;
    }

    let search_term = &args[1];
    let current_dir = env::current_dir().expect("Failed to get current directory");

    search_files(&current_dir, search_term);
}

fn search_files(dir_path: &Path, search_term: &str) {
    if dir_path.is_dir() {
        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() && path.file_stem().unwrap_or_default().to_string_lossy().contains(search_term) {
                        println!("{}", path.display());
                    } else if path.is_dir() {
                        search_files(&path, search_term);
                    }
                }
            }
        }
    }
}
