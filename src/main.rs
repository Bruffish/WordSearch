use std::env;
use std::fs;
use std::error::Error;
use std::path::Path;
use rayon::prelude::*;

/// The main function of the program.
///
/// It collects command line arguments and checks if exactly one argument (the search term) is provided.
/// If not, it prints usage information and exits.
/// Otherwise, it retrieves the current directory and calls `search_files` to start searching recursively.
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <search_term>", args[0]);
        return Ok(());
    }

    let search_term = &args[1];
    let current_dir = env::current_dir()?;
    println!("Searching in: {}", current_dir.display());
    search_files(&current_dir, search_term)?;

    Ok(())
}

/// Recursively searches directories for files whose names contain the specified search term.
///
/// # Arguments
///
/// * `dir_path` - A reference to a `Path` representing the directory to search.
/// * `search_term` - The string to search for in file names.
///
/// # Errors
///
/// Returns an error if reading the directory fails or if any recursive call to `search_files` fails.
fn search_files(dir_path: &Path, search_term: &str) -> Result<(), Box<dyn Error>> {
    if dir_path.is_dir() {
        let entries = fs::read_dir(dir_path)?;
        entries.par_bridge().for_each(|entry| {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() && path.file_stem().unwrap_or_default().to_string_lossy().contains(search_term) {
                    println!("{}", path.display());
                } else if path.is_dir() {
                    if let Err(e) = search_files(&path, search_term) {
                        eprintln!("Error searching directory {}: {}", path.display(), e);
                    }
                }
            }
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
