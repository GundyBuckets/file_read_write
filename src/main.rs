use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let file_path = "example.txt";

    loop {
        println!("\nWhat would you like to do?");
        println!("1. Write to file");
        println!("2. Append to file");
        println!("3. Read from file");
        println!("4. Display file size");
        println!("5. Delete file");
        println!("6. Exit");

        // Get user choice
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let choice = choice.trim();

        match choice {
            "1" => write_to_file(file_path)?,
            "2" => append_to_file(file_path)?,
            "3" => read_from_file(file_path)?,
            "4" => display_file_size(file_path)?,
            "5" => delete_file(file_path)?,
            "6" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }

    Ok(())
}

// Write to the file (overwrites content)
fn write_to_file(file_path: &str) -> io::Result<()> {
    println!("Enter text to write to the file:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    let mut file = File::create(file_path)?;
    file.write_all(input.as_bytes())?;
    println!("Data written to file: {}", file_path);

    Ok(())
}

// Append to the file (keeps existing content)
fn append_to_file(file_path: &str) -> io::Result<()> {
    if !Path::new(file_path).exists() {
        println!("File does not exist. Please write to it first.");
        return Ok(());
    }

    println!("Enter text to append to the file:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input;

    let mut file = File::options().append(true).open(file_path)?;
    file.write_all(input.as_bytes())?;
    file.write_all(b"\n")?;
    println!("Data appended to file: {}", file_path);

    Ok(())
}

// Read from the file
fn read_from_file(file_path: &str) -> io::Result<()> {
    if !Path::new(file_path).exists() {
        println!("File does not exist. Nothing to read.");
        return Ok(());
    }

    let contents = fs::read_to_string(file_path)?;
    println!("File contents:\n{}", contents);

    Ok(())
}

// Display file size
fn display_file_size(file_path: &str) -> io::Result<()> {
    if !Path::new(file_path).exists() {
        println!("File does not exist.");
        return Ok(());
    }

    let metadata = fs::metadata(file_path)?;
    println!("File size: {} bytes", metadata.len());

    Ok(())
}

// Delete the file
fn delete_file(file_path: &str) -> io::Result<()> {
    if !Path::new(file_path).exists() {
        println!("File does not exist.");
        return Ok(());
    }

    fs::remove_file(file_path)?;
    println!("File deleted: {}", file_path);

    Ok(())
}
