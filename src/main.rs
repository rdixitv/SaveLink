use std::{env, fs::File, io::prelude::*, path::Path, process};

enum LnError {
    ArgumentError,
    FileCreationError(String, String),
    FileWriteError(String, String),
}

enum LnSuccess {
    WriteSuccess(String),
    Success,
}

fn write_to_file() -> Result<LnSuccess, LnError> {
    let path = Path::new("link.txt"); // Complete path
    let contents = path.display(); // Filename
    let args: Vec<String> = env::args().collect(); // Get command line arguments

    let mut file = match File::create(&path) {
        Err(why) => {
            return Err(LnError::FileCreationError(
                contents.to_string(),
                why.to_string(),
            ))
        }
        Ok(file) => file,
    };

    match file.write_all(args[2].as_bytes()) {
        Err(why) => {
            return Err(LnError::FileWriteError(
                contents.to_string(),
                why.to_string(),
            ))
        }
        Ok(_) => return Ok(LnSuccess::WriteSuccess(contents.to_string())),
    }
}

// Read from file
fn get_link<P>(filename: P) -> std::io::Result<std::io::Lines<std::io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(std::io::BufReader::new(file).lines())
}

fn display_help() {
    let help: String = String::from(
        "\n
LinkSave - Simple program to save links to a file\n\n
Args:\n
write/add/-n/--new - Write given link to file\n
read/get/-g/--get - Get link from file\n
help/-h/--help - Display this help screen",
    );
    println!("{}", help);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut res: Result<LnSuccess, LnError> = Ok(LnSuccess::Success);
    match args.len() {
        3 => match args[1].as_str() {
            "write" | "add" | "--new" | "-n" => res = write_to_file(),
            _ => res = Err(LnError::ArgumentError),
        },
        2 => match args[1].as_str() {
            "read" | "get" | "--get" | "-g" => {
                if let Ok(lines) = get_link("./link.txt") {
                    for line in lines {
                        if let Ok(link) = line {
                            println!("{}", link)
                        }
                    }
                }
            }
            "help" | "--help" | "-h" => display_help(),
            _ => {
                res = Err(LnError::ArgumentError);
            }
        },
        _ => res = Err(LnError::ArgumentError),
    }

    match res {
        Err(LnError::ArgumentError) => {
            println!("Couldn't understand argument. Showing help screen...");
            display_help();
            process::exit(1);
        }
        Err(LnError::FileCreationError(file, err)) => {
            println!("Couldn't create {}: {}", file, err);
            process::exit(2);
        }
        Err(LnError::FileWriteError(file, err)) => {
            println!("Couldn't write to {}: {}", file, err);
            process::exit(3);
        }
        Ok(LnSuccess::WriteSuccess(file)) => println!("Successfully wrote to {}!", file),
        Ok(LnSuccess::Success) => (),
    }
}
