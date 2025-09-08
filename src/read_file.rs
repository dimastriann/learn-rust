use std::fs;
use std::env;

pub fn open_and_read_file() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = env::current_dir()?;
    // let file_path: &str = "C:\\Dimas\\rust\\learn-rust\\data - copy.txt";
    let file_path: &str = "src\\files\\data.json";
    let full_path = current_dir.join(file_path);
    println!("Full path: {}", full_path.display());

    // let current2 = env::current_dir()?;

    let contents = fs::read_to_string(full_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
    Ok(())
}