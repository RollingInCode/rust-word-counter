use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    // Get the filename from the command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: word_counter <filename>");
        return;
    }
    let filename = &args[1];

    // Read the file contents
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => {
            println!("Failed to open the file");
            return;
        }
    };
    let mut contents = String::new();
    if let Err(_) = file.read_to_string(&mut contents) {
        println!("Failed to read the file");
        return;
    }

    // Count the number of words
    let word_count = contents.split_whitespace().count();
    println!("Word count: {}", word_count);
}
