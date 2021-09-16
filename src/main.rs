use std::env::{current_dir, args};  // For reading run arguments and current directory
use std::io::stdin;                 // User input
//use std::path::PathBuf;             // Special type of string for paths (Path is used elsewhere, and is like a str)

fn main() {
    let args: Vec<String> = args().collect();
    match args.len() {
        1 => {
            menu_mode();
        },
        2 => {
            //TODO this argin is fishy
            let mut argin = args[1].clone().trim().to_string(); // Get file or folder to scan
            println!("Please select the output format:\n1: JSON\n2: CSV");
            let mut choice = String::new();
            stdin().read_line(&mut choice).expect("Error selecting output format");
            let choice = choice.trim();
            match choice {
                "1" => { send_to_scanner(&mut argin) }, // JSON // Really this doesn't need to be &mut, but its to match the stdin method. 
                "2" => { println!("CSV Mode is currently unimplemented") }, // CSV
                 _  => { println!("Goodbye!") }
            }
        },
        3 => {
            match args[1].trim() {
                "-j" => {println!("JSON Mode")}
                "-c" => {println!("CSV Mode")}
                  _  => {println!("Arguments:\n'-j' JSON Output\n'-c' CSV Output")}
            }
        },
        _ => {println!("Invalid number of arguments") }
    }
}
fn menu_mode() {
    let mut cont = true; // Keep going while cont(inue) is true
    while cont {
        let mut choice = String::new();
        println!("Please select your option:\n1. Enter path to scan\n2. View known files\n3. View known detections\n4. View Known Hashes\n5. Delete scan history\n0. Exit");
        stdin().read_line(&mut choice).expect("Error getting menu choice");
        let choice = choice.trim(); // This removes the null terminator, and converts our String to a &str for comparison to our matches
        match choice {
            "1" => { 
                let mut input = get_path_from_stdin();
                send_to_scanner(&mut input);
                press_enter();
            },
            "2" => {
                rfa::json::de_read_all();
                press_enter();
            },
            "3" => {
                rfa::json::de_read_results();
                press_enter();
            },
            "4" => {
                rfa::json::print_hashes();
                press_enter();
            },
            "5" => {
                if are_you_sure() {
                    rfa::json::clear_results().unwrap_or_default();
                    press_enter();
                }
            },
            "0" => cont = false,
            _ => (), // Any other choice, return void
        };
    }
    println!("Exiting...");
}

fn get_path_from_stdin() -> String {
    let mut input = String::new();
    let pwd = current_dir().unwrap();
    println!("Present working directory: {}", &pwd.display());
    stdin().read_line(&mut input).expect("Error reading path from stdin");
    input.trim().to_string()
}

fn send_to_scanner(input: &mut String) { // Converts string into a 'Path' 
    let mut path = current_dir().unwrap();
    path.push(input); // Push the input to the end of the path (pwd = /dev/, input = null, path = /dev/null)
    println!("Scanning: {}", path.display());
    match path.exists() {
        true    => rfa::sample::SampleFile::load_sample(&path).expect("Error reading path"),
        false   => println!("{} doesn't exist.", path.display()),
        };
}

fn press_enter() {
    println!("\nPress any button to continue");
    stdin().read_line(&mut String::new()).expect("Error at press_enter()");
}

fn are_you_sure() -> bool{
    let confirmation;
    let mut input = String::new();
    println!("Are you sure? (y/n)");
    stdin().read_line(&mut input).expect("Error at are_you_sure()");
    let input = input.to_lowercase();
    match input.trim() {
        "y"|"yes" => { 
            //println!("Confirmed");
            confirmation = true;
        },
        _ => { 
            println!("No changes were made."); 
            confirmation = false;
        },
    }
    confirmation
}
