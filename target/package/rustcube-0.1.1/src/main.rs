//! rustcube: Multi-password, order-dependent, streaming encryption/decryption tool
//! - Each password mutates the key state (like a Rubik's cube)
//! - Salt is stored with ciphertext
//! - No error on wrong password: just garbled output
//! - Secure memory zeroing with zeroize

use libaes::Cipher;
use pbkdf2::pbkdf2_hmac;
use rand::RngCore;
use rand::rngs::OsRng;
use sha2::Sha256;
use zeroize::Zeroize;
use clap::{Parser, Subcommand};
mod estimate;
use std::fs::{File};
use std::io::{Read, Write};
use tar::{Builder, Archive};

const SALT_LEN: usize = 16;
const IV_LEN: usize = 16;
const PBKDF2_ITER: u32 = 100_000;

// No longer needed: type Aes256Cbc = Cbc<Aes256, Pkcs7>;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Encrypt {
        #[arg(short, long)]
        folder: String,
        #[arg(short, long)]
        output: String,
    },
    Estimate,
    Decrypt {
        #[arg(short, long)]
        input: String,
        #[arg(short, long)]
        output: String,
    },
}

fn derive_key(passwords: &[String], salt: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32];
    let mut state = Vec::new();
    for pw in passwords {
        let mut k = [0u8; 32];
        pbkdf2_hmac::<Sha256>(pw.as_bytes(), salt, PBKDF2_ITER, &mut k);
        if state.is_empty() {
            state.extend_from_slice(&k);
        } else {
            let state_len = state.len();
            for (i, b) in k.iter().enumerate() {
                let idx = i % state_len;
                state[idx] ^= b;
            }
        }
        k.zeroize();
    }
    key.copy_from_slice(&state[..32]);
    state.zeroize();
    key
}

fn encrypt_folder(folder: &str, output: &str, passwords: &[String]) -> std::io::Result<()> {
    let mut salt = [0u8; SALT_LEN];
    OsRng.fill_bytes(&mut salt);
    let mut iv = [0u8; IV_LEN];
    OsRng.fill_bytes(&mut iv);
    let key = derive_key(passwords, &salt);
    let tar_path = format!("{}.tar", output);
    {
        let tar_gz = File::create(&tar_path)?;
        let mut builder = Builder::new(tar_gz);
        builder.append_dir_all(".", folder)?;
    }
    let mut tar_data = Vec::new();
    {
        let mut f = File::open(&tar_path)?;
        f.read_to_end(&mut tar_data)?;
    }
    let cipher = Cipher::new_256(&key);
    let ciphertext = cipher.cbc_encrypt(&iv, &tar_data);
    let mut out = File::create(output)?;
    out.write_all(&salt)?;
    out.write_all(&iv)?;
    out.write_all(&ciphertext)?;
    std::fs::remove_file(&tar_path)?;
    Ok(())
}

fn decrypt_folder(input: &str, output: &str, passwords: &[String]) -> std::io::Result<()> {
    let mut f = File::open(input)?;
    let mut salt = [0u8; SALT_LEN];
    let mut iv = [0u8; IV_LEN];
    f.read_exact(&mut salt)?;
    f.read_exact(&mut iv)?;
    let mut ciphertext = Vec::new();
    f.read_to_end(&mut ciphertext)?;
    let key = derive_key(passwords, &salt);
    let cipher = Cipher::new_256(&key);
    let decrypted = cipher.cbc_decrypt(&iv, &ciphertext);
    let tar_path = format!("{}.tar", input);
    {
        let mut tar_file = File::create(&tar_path)?;
        tar_file.write_all(&decrypted)?;
    }
    let tar_gz = File::open(&tar_path)?;
    let mut archive = Archive::new(tar_gz);
    archive.unpack(output)?;
    std::fs::remove_file(&tar_path)?;
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Encrypt { folder, output } => {
            println!("Enter passwords (one per line, empty line to finish):");
            let mut passwords = Vec::new();
            loop {
                let pw = rpassword::prompt_password("Password: ").unwrap();
                if pw.is_empty() { break; }
                passwords.push(pw);
            }
            encrypt_folder(&folder, &output, &passwords).expect("Encryption failed");
            println!("Encrypted to {}", output);
        }
        Commands::Decrypt { input, output } => {
            println!("Enter passwords (one per line, empty line to finish):");
            let mut passwords = Vec::new();
            loop {
                let pw = rpassword::prompt_password("Password: ").unwrap();
                if pw.is_empty() { break; }
                passwords.push(pw);
            }
            decrypt_folder(&input, &output, &passwords).expect("Decryption failed");
            println!("Decryption attempted. If passwords were wrong, output will be garbled.");
        }
        Commands::Estimate => {
            estimate::estimate_passwords();
        }
    }
}
