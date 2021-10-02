use sha2::{Sha256, Digest};
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use serde::{Deserialize, Serialize};
use infer;

#[derive(Serialize, Deserialize, Debug)]
pub struct SampleFile {
    pub hash: String, // really this is a Sha256
    file_stem: String,
    extension: String,
    pub results: String,
}

impl SampleFile {
    pub fn load_sample(path: &Path) -> io::Result<()>{
        match path.is_dir() {
            false =>    
                SampleFile::catalog_file(&path),
            true => 
                for f in path.read_dir().expect("Error reading contents of directory") {
                    if let Ok(f) = f { 
                        SampleFile::load_sample(&f.path())?; // Reads folders recursively until we get a file.
                        // TODO Maybe make sure this doesn't cause a stack overflow
                    } 
                },
        };
        Ok(())
    }

    fn sha256sum(path: &Path) -> String {
        let mut file = match File::open(&path) {
            Err(why) =>
                panic!("Couldn't open {:?}: {}", path.display(), why), // If we can't open it, panic and say why
            Ok(file) => 
                file, // else, open the file
        };
        let mut buffer = Vec::new();
        match file.read_to_end(&mut buffer) { // read the entirety of the file into a vector of bytes (unsigned 8-bit integers: or u8)
            Err(why) => panic!("File was opened, but could not be read: {}", why),
            Ok(buffer) => buffer,
        };
        let hash: String = format!("{:x}", Sha256::digest(&buffer));
        hash
    }

    fn infer_mime(path:&Path) -> String {
        let infer_mime = infer::get_from_path(&path).expect("error reading");
        let results = match infer_mime {
            Some(_x) => infer_mime.unwrap().to_string(),
            None => String::from("N/A"),
        };
        results
    }

    fn catalog_file(path: &Path) {
        // This looks hairy so let me explain: 
        // By default, these path variables are OsStrs, variable length, OS-sensitive string slices
        // unwrap_or_default(): removes the Option (Rust's way to avoid null values, there can be Some or None data)
        // to_string_lossy(): converts the OsStr to a Cow (Copy-On-Write smart pointer), 'lossy' meaning we might lose data
        // when we convert from the OS's format to UTF-8 (All strings in rust must be valid UTF-8).
        let file_stem = path.file_stem().unwrap_or_default().to_string_lossy().to_string(); // filename w/o extension
        let extension = path.extension().unwrap_or_default().to_string_lossy().to_lowercase().to_string(); 
        let hash = SampleFile::sha256sum(&path);
        let results = SampleFile::infer_mime(&path);
        let sample = SampleFile { hash, file_stem, extension, results };
        super::json::serialize(&sample).expect("Failed to serialize");
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
