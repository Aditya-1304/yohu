use std::{fs};
use std::io::{self, Write};

fn main() {

    let text_to_process = loop {
        println!("\nHow would you like to provide the text?");
        println!(" 1: Type text directly");
        println!(" 2: Provide a file path");
        let choice = get_user_input("Enter your choice (1 or 2): ");

        match choice.as_str() {
            "1" => {
                break get_user_input("Please enter the text to process: ");
            }
            "2" => {
                let file_path = get_user_input("Please enter the path to the file: ");
                match fs::read_to_string(&file_path) {
                    Ok(contents) => break contents,
                    Err(e) => {
                        eprintln!("\nError: Failed to read file '{}'. Reason: {}", file_path, e);
                    }
                }
            }
            _ => {
                eprintln!("\nError: Invalid choice. Please enter 1 or 2.");
            }
        }
    };
   
   let shift_amount: i16 = loop {
    let shift_str = get_user_input("\nEnter the shift amount (for ex: 3 for encrypt and -3 for decrypt): ");
    match shift_str.parse() {
        Ok(num) => break num,
        Err(_) => {
            eprintln!("\nError: Invalid shift amount. Please enter a valid integer.")
        }
    }
   };

    println!("\n-------------------------------------------------");
    println!("Original Text Length: {} characters", text_to_process.len());
    println!("Shift Amount:         {}", shift_amount);
    println!("-------------------------------------------------");

    // --- 3. Process the text and print the result ---
    let processed_text = caesar_cipher(&text_to_process, shift_amount);
    println!("\nProcessed Text:\n");
    println!("{}", processed_text);

}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn caesar_cipher(text:&str, shift:i16) -> String {
    text.chars()
        .map(|c|{
            if c.is_alphabetic() {
                let base = if c.is_lowercase() {'a'} else {'A'};

                let current_pos = c as u8 - base as u8;

                let new_pos = (current_pos as i16 + shift).rem_euclid(26);

                (base as u8 + new_pos as u8) as char
            } else {
                c
            }
        })
        .collect()
}