# Yohu 🔐

A versatile command-line cryptography tool implementing multiple cipher algorithms for educational and practical use.

## Features

- **Caesar Cipher**: Classic shift cipher with customizable shift values
- **Vigenère Cipher**: Polyalphabetic cipher using keyword-based encryption/decryption
- **AES-128-CBC**: Modern symmetric encryption with PKCS7 padding
- **Caesar Cipher Cracking**: Automatic cryptanalysis using Chi-squared frequency analysis
- **Flexible Input**: Support for direct text input or file-based processing
- **File I/O**: Save results to files for further use

## Installation

### From Crates.io 
```bash
cargo install yohu
```

### From Source
```bash
git clone <your-repository-url>
cd yohu
cargo build --release
```

## Usage

Run the interactive CLI:
```bash
cargo run
```

The application will guide you through:
1. Choosing your cipher method
2. Providing necessary parameters (keys, shift values, etc.)
3. Selecting input method (direct text or file)
4. Processing and displaying results
5. Optionally saving output to a file

### Cipher Options

#### 1. Caesar Cipher
- **Purpose**: Simple substitution cipher
- **Input**: Shift amount (positive or negative integer)
- **Example**: Shift of 3 transforms 'A' → 'D'

#### 2. Vigenère Cipher
- **Purpose**: Polyalphabetic substitution cipher
- **Input**: Keyword (alphabetic characters)
- **Mode**: Encryption or decryption
- **Example**: Keyword "KEY" creates repeating shift pattern

#### 3. AES-128-CBC
- **Purpose**: Modern symmetric encryption
- **Input**: 
  - 16-character secret key
  - 16-character initialization vector (IV)
- **Mode**: Encryption or decryption
- **Output**: Hex-encoded ciphertext for encryption

#### 4. Caesar Cipher Cracking
- **Purpose**: Automatic cryptanalysis
- **Method**: Chi-squared frequency analysis against English letter frequencies
- **Output**: Best guess decryption and estimated shift key

## Examples

### Caesar Cipher Encryption
```
Input: "Hello World"
Shift: 3
Output: "Khoor Zruog"
```

### Vigenère Cipher
```
Input: "HELLO"
Keyword: "KEY"
Output: "RIJVS"
```

### AES-128-CBC
```
Input: "Secret Message"
Key: "MySecretKey12345"
IV: "MyInitVector1234"
Output: Hex-encoded ciphertext
```

## Dependencies

- `aes` - AES block cipher implementation
- `cbc` - Cipher Block Chaining mode
- `hex` - Hexadecimal encoding/decoding

## Security Notice

⚠️ **Important**: This tool is designed for educational purposes and experimentation. For production use:

- Use cryptographically secure random number generators for keys and IVs
- Consider more robust key derivation functions
- Implement proper key management practices
- Classical ciphers (Caesar, Vigenère) are not secure for real-world use

## Contributing

Contributions are welcome! Please feel free to submit issues, feature requests, or pull requests.

### Development Setup

1. Clone the repository
2. Install Rust (1.70+ recommended)
3. Run tests: `cargo test`
4. Build: `cargo build`

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Frequency analysis implementation based on standard English letter frequencies
- AES implementation uses the RustCrypto project's `aes` and `cbc` crates

---

**Yohu** - Making cryptography accessible and educational! 🚀