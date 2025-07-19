use std::{fs};
use std::io::{self, Write};
use std::collections::HashMap;

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
        println!("  3: Crack Caesar Cipher (auto-decrypt)");
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
    let mut frequencies = HashMap::new();

    for c in text.chars().filter(|c| c.is_alphabetic()) {
        *frequencies.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
    }

    let most_frequent_char = frequencies
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(c,_)| c)
        .unwrap_or('e');

    let assumed_e = 'e' as i16;

    let most_frequent_code = most_frequent_char as i16;

    let guessed_shift = (most_frequent_code - assumed_e).rem_euclid(26);

    let decrypt_shift = -guessed_shift;

    let decrypted_text = caesar_cipher(text, decrypt_shift);

    (decrypted_text, guessed_shift)
}