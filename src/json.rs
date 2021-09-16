use std::fs::{File, OpenOptions, read_to_string, remove_file};
use std::path::Path;
use std::env::current_dir;
use serde_json;
use crate::sample::SampleFile;

const DEF_FILE: &str = "results.json";

pub fn serialize(s: &SampleFile) -> std::io::Result<()> { // Push SampleFile into the json results file
    check_if_results_exists();
    if is_sample_unique(&s.hash) == true {
        let buffer = OpenOptions::new() // OpenOptions makes it easier to append than File operations.
            .append(true)
            .open(DEF_FILE)
            .unwrap();
        serde_json::to_writer_pretty(buffer, &s).unwrap(); // I'm leaving these unwrappers to panic on error because we don't want to write incorrect data.
        s.print();
    } else {
        println!("{} is a duplicate.\n", s.name());
    }
    Ok(()) // This returns a result, either void or error, so that we can error handle if the io operations fail.
}

fn deserialize() -> Vec<SampleFile> { // Return a vector of SampleFiles read from the json file
//pub fn deserialize() {
    check_if_results_exists();
    let f = read_to_string(DEF_FILE).unwrap(); // Once again if any of these unwraps fail we won't have proper data anyway, so we might as well crash here rather than later.
    let mut known_samples = Vec::new();
    let raw_objs: Vec<&str> = f.split_inclusive("}").collect(); // Split the str up into a vec so that the deserializer can parse each result. 
    for defs in raw_objs {
        let s: SampleFile = serde_json::de::from_str(defs).unwrap(); // Create a temp SampleFile 
        known_samples.push(s);
        //println!("{:?}", known_samples.last().unwrap());
        //known_samples.last().unwrap().print();
    }
    known_samples
}
pub fn de_read_all() {
    let samples = deserialize();
    for s in samples {
        s.print();
    }
}

pub fn de_read_results() {
    let samples = deserialize();
    for s in samples {
        if !(s.results.is_empty()) {
            s.print();
        }
    }
}

fn is_sample_unique(hash: &String) -> bool {
    let hashes = get_known_hashes();
    let mut unique = true;
    if hashes.contains(&hash) {
        unique = false;
        //println!("{} is a duplicated SHA2");
    }
    unique
}

pub fn get_known_hashes() -> Vec<String> { // Skip the deserializer and simply read the raw hash strings into a vec.
    let mut hashes = Vec::new();
    if check_if_results_exists() == true {
        let f = read_to_string(DEF_FILE).unwrap_or_default(); 
        let hash_line = f.lines().skip(1).step_by(5); // Start at the second line, move up by 5 lines each time. This is where the hash value is.
        for line in hash_line {
            hashes.push(
                line.
                strip_prefix("  \"hash\": \"").unwrap_or_default(). // This is more forgiving than the deserializer, since we can just return an empty string instead of a broken struct.
                strip_suffix("\",").unwrap_or_default() // These strips leave us with the SHA2 sum and nothing else.
                .to_string()); // From &str to String
            //println!("{}", hashes.last().unwrap());
        }
    }
    //println!("Before: {:?}", hashes.len());
    hashes.sort();
    hashes.dedup();
    //println!("Loaded {:?} unique hashes.", hashes.len());
    hashes
}

pub fn print_hashes() {
    let hashes = get_known_hashes();
    for i in hashes {
        println!("{}", i)
    }
}

fn check_if_results_exists() -> bool { // Check if we need to create the DEF_FILE
    let mut does_exist = false;
    match Path::new(DEF_FILE).exists() {
        true => does_exist = true,
        false => match create_results() {
            Ok(()) => (),
            Err(why) => panic!("Couldn't create {}\nError: {}", DEF_FILE, why),
        },
    };
    does_exist
}

fn create_results() -> std::io::Result<()> { // Create the DEF_FILE if we need to, return an IO error if we cant.
    let pwd = current_dir()?;
    println!("{} not found, creating in:\npresent directory: {:?} ", DEF_FILE, pwd.display());
    File::create(DEF_FILE)?;

    Ok(()) // Return Result<void>, meaning we have to handle if we get an Err back.
}

pub fn clear_results() -> std::io::Result<()> {
    //let pwd = current_dir()?;
    remove_file(DEF_FILE)?;
    println!("{} removed successfully", DEF_FILE);
    Ok(())
}
