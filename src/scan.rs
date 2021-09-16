use infer;
pub fn scanner(buffer: Vec<u8>) -> String {
    let mut result = String::from("");
    result.push_str(infer_type(&buffer));
    result.trim().to_string()
}

fn infer_type(buffer: &Vec<u8>) -> & str {
    let magic_number = infer::get(&buffer);
    let mut result = "N/A";
    match magic_number {
        Some(magic_number) => result = magic_number.mime_type(),
        None => println!("Unable to infer mime type.")
    }
    println!("{}", result);
    result
}
