# rustcube

[![Release](https://img.shields.io/github/v/release/orchestrate-solutions/rustcube?label=release)](https://github.com/orchestrate-solutions/rustcube/releases)
[![Crates.io](https://img.shields.io/crates/v/rustcube?label=crates.io)](https://crates.io/crates/rustcube)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://github.com/orchestrate-solutions/rustcube/blob/main/LICENSE)

Multi-password, order-dependent, encryption/decryption tool with secure memory handling. 

Think Rubik’s Cube meets combination lock. Every password turns the cube. Only the exact twist sequence lands on the solving state that opens the box.

## Features
- **Multi-password, order-dependent**: Any number of passwords, order matters, case sensitive. Only the correct sequence unlocks the data everything else returns non-sensical output.
- **Streaming encryption/decryption**: Handles large files/folders efficiently.
- **Salted key derivation**: Each encryption is unique, even with the same passwords.
- **No error on wrong password**: Decryption always produces output; wrong passwords yield unusable (garbled) data.
- **Secure memory handling**: Uses the `zeroize` crate to clear sensitive data from memory.
- **Tar-based archiving**: Folders are packed/unpacked using tar for cross-platform compatibility.
- **CLI interface**: Easy to use, scriptable, and ready for automation.

## Usage


### Install from crates.io

```
cargo install rustcube
```

### Build from source

```
cargo build --release
```

### Encrypt a Folder

```
cargo run --release -- encrypt --folder <FOLDER_TO_ENCRYPT> --output <OUTPUT_FILE>
```
- You will be prompted for passwords (one per line, empty line to finish).
- The output file will contain the salt, IV, and encrypted data.

### Decrypt a File

```
cargo run --release -- decrypt --input <ENCRYPTED_FILE> --output <OUTPUT_FOLDER>
```
- You will be prompted for passwords (one per line, empty line to finish).
- If the passwords and order are correct, the folder will be restored.
- If not, the output will be garbled (no error is shown).

## How It Works

## Multi-Password Usability Advantage

Instead of relying on a single massive password, rustcube lets you use several smaller, memorable passwords in a strict order. This makes it easier to remember and type, while still providing extremely strong security. The number of possible combinations grows rapidly with each additional password and the order in which they are entered, making brute-force attacks much harder.

For example, 4 passwords of 6 characters each (with order sensitivity) can be as strong or stronger than a single 24-character password, depending on the entropy of each password and the total number of possible combinations.

**Tip:** Use unique, non-trivial passwords and avoid common words or sequences for best results.

## Planned Feature: Brute-Force Calculator

In a future version, rustcube will include a brute-force calculator. When you set your passwords, it will estimate how long it would take to brute-force your chosen combination using:

- A high-spec modern machine or cluster (e.g., billions of guesses per second)
- A hypothetical quantum computer (using Grover's algorithm, which can halve the effective keyspace)

This feedback will help you choose a password sequence that balances usability and security for your needs.

- **Key Derivation**: Each password is used to mutate the key state (like turning a Rubik's cube). The final key is used for AES-256 encryption/decryption.
- **Salt**: A random salt is generated for each encryption and stored with the ciphertext. This ensures uniqueness and prevents rainbow table attacks.
- **IV**: A random IV is generated for each encryption and stored with the ciphertext.
- **Memory Security**: All sensitive data (passwords, keys, intermediate states) are zeroed from memory after use.
- **No Feedback on Failure**: Decryption always produces output. If the key is wrong, the output is just random data.

## Security Notes
- Passwords and their order are never stored.
- The salt and IV are not secrets; they are stored with the encrypted file.
- For maximum security, use strong, unique passwords and keep them safe.
- The tool is designed for local, user-controlled encryption and decryption. You can use it in the cloud if you wish, but its primary intent is to let a user keep private, secured data alongside public data (such as in a repo), so you can pull your repo from anywhere and always access your own secure data—while keeping it inaccessible to others. You control if, when, and how you share your secrets. For cloud or multi-user scenarios, review your threat model and use at your own discretion.

## Dependencies
- [aes](https://crates.io/crates/aes)
- [block-modes](https://crates.io/crates/block-modes)
- [block-padding](https://crates.io/crates/block-padding)
- [pbkdf2](https://crates.io/crates/pbkdf2)
- [rand](https://crates.io/crates/rand)
- [zeroize](https://crates.io/crates/zeroize)
- [clap](https://crates.io/crates/clap)
- [tar](https://crates.io/crates/tar)
- [hex](https://crates.io/crates/hex)

## Example

```
# Encrypt
cargo run --release -- encrypt --folder secrets --output secrets.enc

# Decrypt
cargo run --release -- decrypt --input secrets.enc --output secrets_restored
```

## License
MIT

---

God willing, this tool will help you keep your secrets safe and your workflow efficient.
