use std::{
    env,
    fs::File,
    io::prelude::*,
    path::Path
};

fn write_to_file() {
    let path = Path::new("link.txt"); // Complete path
    let contents = path.display(); // Filename
    let args: Vec<String> = env::args().collect(); // Get command line arguments


    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", contents, why),
        Ok(file) => file,
    };

    match file.write_all(args[2].as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", contents, why),
        Ok(_) => println!("Successfully wrote to {}", contents)
    }

}

// Read from file
fn get_link<P>(filename: P) -> std::io::Result<std::io::Lines<std::io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(std::io::BufReader::new(file).lines())
}

fn display_help() {
    let help: String = String::from("\n
LinkSave - Simple program to save links to a file\n\n
Args:\n
write/add/-n/--new - Write given link to file\n
read/get/-g/--get - Get link from file\n
help/-h/--help - Display this help screen");
    println!("{}", help);
}

fn main() {
    let args: Vec<String> = env::args().collect();   
    if args[1] == "write" || args[1] == "add" || args[1] == "--new" || args[1] == "-n" {
        write_to_file();
    }
    else if args[1] == "read" || args[1] == "get" || args[1] == "--get" || args[1] == "-g" {
        if let Ok(lines) = get_link("./link.txt") {
            for line in lines {
                if let Ok(link) = line {
                    println!("{}", link)
                }
            }
        }
    }
    else if args[1] == "help" || args[1] == "--help" || args[1] == "-h" {
        display_help();
    }
    else {
        println!("Couldn't understand argument. Showing help screen...");
        display_help();
    }
}
