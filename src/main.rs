use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() !=3 {
        eprintln!("Error: Incorrect number of arguments.");
        eprintln!("Usage: {}\"<text to encrypt>\" <shift amount>", args[0]);

        process::exit(1)
    }

    let text_to_process = &args[1];

    let shift_amount: i16 = match args[2].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: The shift amount must be a valid integer.");
            eprintln!("Usage: {} \"<text to encrypt>\" <shift amount>", args[0]);
            process::exit(1);
        }
    };

    println!("--- CipherCraft v0.1: Caesar Cipher ---");
    println!();
    println!("Original Text:  {}", text_to_process);
    println!("Shift Amount:   {}", shift_amount);
    println!("-----------------------------------------");

    // --- Encryption ---
    let encrypted_text = caesar_cipher(text_to_process, shift_amount);
    println!("Encrypted Text: {}", encrypted_text);

    // --- Decryption ---
    // Decryption is just encryption with a negative shift.
    let decrypted_text = caesar_cipher(&encrypted_text, -shift_amount);
    println!("Decrypted Text: {}", decrypted_text);
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