use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let path = match args.last() {
        Some(path) => path,
        None => {
            // No path provided
            eprintln!("Error: Please provide a directory path as an argument.");
            std::process::exit(1);
        }
    };

    let paths = match fs::read_dir(path) {
        Ok(paths) => paths,
        Err(err) => {
            // Error reading directory
            eprintln!("Error: Failed to read directory: {}", err);
            std::process::exit(1);
        }
    };

    for path in paths {
        let path = path.unwrap(); // Unwrap the Result since we handled potential errors above
        println!("Name: {}", path.path().display());
    }
}
