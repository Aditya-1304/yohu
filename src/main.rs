use std::{fs};
use std::io::{self, Write};
// use std::collections::HashMap;

enum Cipher {
    Caesar(i16),
    Vigenere { keyword: String, decrypt: bool},
    CrackCaesar,
}

impl Cipher {
    fn process(&self, text: &str) -> String {
        match self {
            Cipher::Caesar(shift) => {
                println!("\nProcessing with Caesar Cipher (shift: {})...", shift);
                caesar_cipher(text, *shift)
            }
            Cipher::Vigenere { keyword,decrypt } => {
                let mode_str = if *decrypt {"decrypt"} else {"encrypt"};
                println!("\nProcessing with Vigenère cipher (keyword: '{}', mode: {})...", keyword, mode_str);
                vigenere_cipher(text, keyword, *decrypt)
            }
            Cipher::CrackCaesar => {
                println!("\nAttempting to crack Caesar cipher using frequency analysis...");
                let (decrypted_text, guessed_shift) = crack_caesar(text);
                println!("Guessed shift key was: {}", guessed_shift);
                decrypted_text
            }
        }
    }
}

fn main() {

    let chosen_cipher = loop {
        println!("\nPlease choose a cipher: ");
        println!(" 1. Caesar Cipher (Shift by number)");
        println!(" 2. Vigenere Cipher (Shift by keyword)");
        println!("-------------------------------------------------");
        println!(" Yohu also provides a method to crack Caesar Cipher");
        println!("\n 3: Crack Caesar Cipher (auto-decrypt)");
        let choice = get_user_input("Enter your choice (1, 2, or 3): ");
        match choice.as_str() {
            "1" => {
                let shift_amount: i16 = loop {
                    let shift_str = get_user_input("\nEnter the Caesar shift amount (e.g., 3 or -3): ");
                    match shift_str.parse() {
                        Ok(num) => break num,
                        Err(_) => eprintln!("\nError: Invalid shift amount. Please enter a valid integer."),
                    }
                };
                break Cipher::Caesar(shift_amount);
            }
            "2" => {
                let keyword = loop {
                    let key = get_user_input("\nEnter the Vigenere keyword: ");
                    if key.chars().any(|c| c.is_alphabetic()) {
                        break key;
                    }
                    eprintln!("\nError: Keyword must contain at least one alphabetic character.");
                };
                let decrypt = loop {
                    let mode = get_user_input("Encrypt or Decrypt? (e/d): ").to_lowercase();
                    if mode == "e" { break false; }
                    if mode == "d" { break true; }
                    eprintln!("\nError: Invalid mode. Please enter 'e' or 'd'.");
                };
                break Cipher::Vigenere { keyword, decrypt }
            }
            "3" => {
                break Cipher::CrackCaesar;
            }
            _ => eprintln!("\nError: Invalid choice. Please enter 1, 2 or 3."),
        }
    };

    let text_to_process = loop {
        println!("\nHow would you like to provide the text?");
        println!(" 1: Type text directly");
        println!(" 2: Provide a file path");
        let choice = get_user_input("Enter your choice (1 or 2): ");

        match choice.as_str() {
            "1" => {
                break get_user_input("Please enter the text to encrypt/decrypt: ");
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

    let processed_text = chosen_cipher.process(&text_to_process);
    println!("\n-------------------------------------------------");
    println!("\nResult:\n");
    println!("{}", processed_text);
    println!("\n-------------------------------------------------");

    loop {
        let save_choice = get_user_input("\nSave result to a file? (y/n): ").to_lowercase();
        match save_choice.as_str() {
            "y" | "yes" => {
                let filename = get_user_input("Enter filename to save as: ");
                match fs::write(&filename, &processed_text) {
                    Ok(_) => {
                        println!("Successfully saved to '{}'.", filename);
                        break;
                    }
                    Err(e) => {
                        eprintln!("Error: Failed to save file. Reason: {}",e);
                    }
                }
            }
            "n" | "no" => {
                println!("Exiting.");
                break;
            }
            _ => {
                eprintln!("Invalid input. Please enter 'y' or 'n'.");
            }
        }
    }

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

fn vigenere_cipher(text: &str, keyword: &str, decrypt: bool) -> String {
    let mut key_chars = keyword.chars().filter(|c| c.is_alphabetic()).cycle();

    text.chars()
        .map(|c| {
            if c.is_alphabetic() {
                let key_char = match key_chars.next() {
                    Some(k) => k,
                    None => return c,
                };

                let base = if c.is_lowercase() {'a'} else {'A'};

                let shift = key_char.to_ascii_lowercase() as u8 - b'a';

                let effective_shift = if decrypt {-(shift as i16)} else { shift as i16 };

                let current_pos = c as u8 - base as u8;

                let new_pos = (current_pos as i16 + effective_shift).rem_euclid(26);
                (base as u8 + new_pos as u8) as char
            } else {
                c
            }
        })
        .collect()
}


fn crack_caesar(text: &str) -> (String, i16) {
    // Standard English letter frequencies (source: Wikipedia)
    const ENGLISH_FREQS: [f64; 26] = [
        0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015, 
        0.06094, 0.06966, 0.00153, 0.00772, 0.04025, 0.02406, 0.06749, 
        0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056, 0.02758, 
        0.00978, 0.02360, 0.00150, 0.01974, 0.00074, 
    ];

    let mut best_guess = ("".to_string(), -1);

    let mut min_score = f64::MAX;

    let text_alphabetic_chars: Vec<char> = text
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect();
    let total_chars = text_alphabetic_chars.len() as f64;
    
    if total_chars == 0.0 {
        return (text.to_string(), 0); 
    }
    
    for shift_guess in 0..26 {
        let decrypted_text = caesar_cipher(text, -shift_guess);
        
        let mut observed_counts = [0.0; 26];
        for c in decrypted_text.chars().filter(|c| c.is_alphabetic()) {
            let index = (c.to_ascii_lowercase() as u8 - b'a') as usize;
            observed_counts[index] += 1.0;
        }
        
        let mut current_score = 0.0;
        for i in 0..26 {
            let expected_count = ENGLISH_FREQS[i] * total_chars;
            let difference = observed_counts[i] - expected_count;
            current_score += difference * difference / expected_count;
        }

        if current_score < min_score {
            min_score = current_score;
            best_guess = (decrypted_text, shift_guess);
        }
    }
    best_guess
}