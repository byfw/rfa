use sha256::{digest_bytes};
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct SampleFile {
    pub hash: String, // really this is a Sha256
    file_stem: String,
    extension: String,
    pub results: String,
}
    
impl SampleFile {
    pub fn load_sample(path: &Path) -> io::Result<()>{ // str is an unsized String prim (string slice)
        match path.is_dir() {
            false => SampleFile::catalog_file(&path),   // false: is not a dir, assume its a file.
            true =>                                     // true: for every file in dir, try to catalog every file.
                for f in path.read_dir().expect("Error reading contents of directory") {
                    if let Ok(f) = f { 
                        SampleFile::load_sample(&f.path())?; // Reads folders recursively until we get a file. 
                    } 
                },
        };
        Ok(())
    }

    fn catalog_file(path: &Path) { // str is an unsized String prim (string slice)
        // This looks hairy so let me explain: 
        // By default, these path variables are OsStrs, variable length, OS-sensitive string slices
        // unwrap_or_default(): removes the Option (Rust's way to avoid null values, there can be Some or None data)
        // to_string_lossy(): converts the OsStr to a Cow (Copy-On-Write smart pointer), 'lossy' meaning we might lose data
        // when we convert from the OS's format to UTF-8 (All strings in rust must be valid UTF-8).
        let file_stem = path.file_stem().unwrap_or_default().to_string_lossy().to_string(); // filename w/o extension
        let extension = path.extension().unwrap_or_default().to_string_lossy().to_string(); 
        let info = SampleFile::read_buffer(&path); // Get info and the scan results as a tuple, info.0 is the has, info.1 is results
        let sample = SampleFile { hash: info.0, file_stem, extension, results: info.1 };  
        super::json::serialize(&sample).expect("Failed to serialize");
    }

    fn read_buffer(path: &Path) -> (String, String) { 
        let mut file = match File::open(&path) { // Attempt to load the file by its path
            Err(why) => panic!("Couldn't open {:?}: {}", path.display(), why), // If we can't open it, panic and say why
            Ok(file) => file, // else, open the file
        };
        let mut buffer = Vec::new();
        match file.read_to_end(&mut buffer) { // read the entirety of the file into a vector of bytes (unsigned 8-bit integers: or u8)
            Err(why) => panic!("File was opened, but could not be read: {}", why),
            Ok(buffer) => buffer,
        };

        let hash = digest_bytes(&buffer);           // use the sha256 library to hash the file, outputs a String
        let results = super::scan::scanner(buffer); // consume the buffer vec
        (hash, results)                             // return sha2 and any scan results to build the struct
    }

    pub fn print(&self) {           // Make a print method for our struct so it can be called like sample.print()
        if self.extension == "" {   // No extension, don't print the .
            println!("\n[ {} ] SHA256SUM: {}", self.file_stem, self.hash);
        } else {
            println!("\n[ {}.{} ] - SHA256SUM: {}", self.file_stem, self.extension, self.hash);
        }
        if !self.results.is_empty() {
            println!("RESULTS: {}", self.results);
        }
    }

    pub fn name(&self) -> String {  // Return filename as string
        let name;
        match &self.extension.is_empty() {
            true    => name = String::from(&self.file_stem),
            false   => name = String::from(&self.file_stem) + "." + &self.extension,
        }
        name
    }
} 
