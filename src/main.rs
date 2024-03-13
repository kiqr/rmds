use std;
use walkdir::WalkDir;

fn main() {
    // Get the starting directory from the command line arguments. If none, use the current directory.
    let start_dir = std::env::args().nth(1).unwrap_or(".".to_string());

    println!("Recursively searching for .DS_Store files in {start_dir}.\n");

    // Create an array to hold all the paths of the .DS_Store files found.
    let mut ds_store_paths = Vec::new();

    // Search recursively for .DS_Store files from the current directory
    // and print the path of each file found.
    // The `walkdir` crate is used to perform the recursive search.
    for entry in WalkDir::new(start_dir) {
        let entry = entry.unwrap();
        if entry.file_name() == ".DS_Store" {
            ds_store_paths.push(entry.path().to_path_buf());
        }
    }

    // Print all the paths of the .DS_Store files found and ask the user if they want to delete them.
    for path in &ds_store_paths {
        println!("Found .DS_Store file at: {:?}", path);
    }

    // Ask the user if they want to delete the .DS_Store files found.
    // If the user enters "y" or "yes", delete the files.
    // If the user enters "n" or "no", do not delete the files.
    // If the user enters anything else, ask again.
    loop {
        println!("\nDo you want to delete the .DS_Store files? (y/n)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => {
                for path in &ds_store_paths {
                    delete_file(path);
                }
                break;
            }
            "n" | "no" => {
                println!("Did not delete any .DS_Store files.");
                break;
            }
            _ => {
                println!("Invalid input. Please enter 'y' or 'n'.");
            }
        }
    }
}

fn delete_file(path: &std::path::PathBuf) {
    // Attempt to delete the file at the given path.
    match std::fs::remove_file(path) {
        Ok(_) => println!("Deleted .DS_Store file at: {:?}", path),
        Err(e) => eprintln!("Error deleting .DS_Store file at {:?}: {}", path, e),
    }
}
