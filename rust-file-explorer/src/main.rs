use std::{env::{self, set_current_dir}, fs, io::Read, process};

// lists all the files and directories
fn list_contents(path: Option<&str>) {
    let path = path.unwrap_or(".");
    let entries = fs::read_dir(path).unwrap_or_else(|err| {
        eprintln!("Failed to read contents of directory '{}': {}", path, err);
        process::exit(1);
    });

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        println!("{}", path.display());
    }
}

// [NOT WORKING] changes the current working directory
fn change_directory(path: &str) {
    set_current_dir(path).unwrap_or_else(|err| {
        eprintln!("Failed to change directory '{}': {}", path, err);
        process::exit(1);
    });
    println!("Changed directory to '{}'", path);
}

// [NOT WORKING FOR DIRS] removes a file
fn remove_file(path: &str) {
    fs::remove_file(path).unwrap_or_else(|err| {
        eprintln!("Failed to remove file '{}': {}", path, err);
        process::exit(1);
    });
    println!("Removed file '{}'", path);
}

// read and prints the content of a file
fn read_content(path: &str) {
    let mut file = fs::File::open(path).unwrap_or_else(|err| {
        eprintln!("Failed to open file '{}': {}", path, err);
        process::exit(1);
    });

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap_or_else(|err| {
        eprintln!("Failed to read file '{}': {}", path, err);
        process::exit(1);
    });

    println!("{}", content);
}

// creates a new directory
fn make_dir(path: &str) {
    fs::create_dir(path).unwrap_or_else(|err| {
        eprintln!("Failed to create directory '{}': {}", path, err);
        process::exit(1);
    });
    println!("Created directory '{}'", path);
}

/*
    expected behaviour
        cargo run ls .
        cargo run cd /path/to/directory
        cargo run cat /path/to/file
        cargo run rm /path/to/file
        cargo run mkdir /path/to/new_directory
        [TODO:] cargo run mv /path/to/source /path/to/dest
*/
fn main() {
    // collect the command line arguments
    let args: Vec<String> = env::args().collect();
    
    // check if there are enough arguments
    if args.len() < 2 {
        println!("Not enough arguments");
        return;
    }

    let command = &args[1];
    match command.as_str() {
        "ls" => {
            let path = if args.len() > 2 { Some(args[2].as_str()) } else { None };
            list_contents(path);
        },
        "cd" => {
            if args.len() < 3 {
                eprintln!("Usage: cd <path>");
                process::exit(1);
            }
            change_directory(&args[2]);
        },
        "cat" => {
            if args.len() < 3 {
                eprintln!("Usage: cat <file>");
                process::exit(1);
            }
            read_content(&args[2]);
        },
        "rm" => {
            if args.len() < 3 {
                eprintln!("Usage: rm <file>");
                process::exit(1);
            }
            remove_file(&args[2]);
        },
        "mkdir" => {
            if args.len() < 3 {
                eprint!("Usage: mkdir <path>");
                process::exit(1);
            }
            make_dir(&args[2]);
        },
        _ => println!("Invalid command"),
    }
}