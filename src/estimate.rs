use std::io::{self, Write};

fn get_charset_size(pw: &str) -> usize {
    let mut has_lower = false;
    let mut has_upper = false;
    let mut has_digit = false;
    let mut has_symbol = false;
    for c in pw.chars() {
        if c.is_ascii_lowercase() { has_lower = true; }
        else if c.is_ascii_uppercase() { has_upper = true; }
        else if c.is_ascii_digit() { has_digit = true; }
        else { has_symbol = true; }
    }
    let mut size = 0;
    if has_lower { size += 26; }
    if has_upper { size += 26; }
    if has_digit { size += 10; }
    if has_symbol { size += 33; }
    size
}

fn estimate_entropy(pw: &str, is_dict: bool) -> f64 {
    if is_dict {
        // Penalize dictionary words: assume 10 bits
        10.0
    } else {
        let charset = get_charset_size(pw);
        (pw.len() as f64) * (charset as f64).log2()
    }
}

pub fn estimate_passwords() {
    println!("Enter passwords (one per line, empty line to finish):");
    let mut passwords = Vec::new();
    let mut entropies = Vec::new();
    loop {
        print!("Password: "); io::stdout().flush().unwrap();
        let mut pw = String::new();
        io::stdin().read_line(&mut pw).unwrap();
        let pw = pw.trim().to_string();
        if pw.is_empty() { break; }
        print!("Is this a dictionary word? (y/N): "); io::stdout().flush().unwrap();
        let mut dict = String::new();
        io::stdin().read_line(&mut dict).unwrap();
        let is_dict = dict.trim().to_lowercase() == "y";
        let entropy = estimate_entropy(&pw, is_dict);
        passwords.push(pw);
        entropies.push(entropy);
    }
    if passwords.is_empty() {
        println!("No passwords entered.");
        return;
    }
    let total_entropy: f64 = entropies.iter().sum();
    let total_keyspace = 2f64.powf(total_entropy);
    let guesses_per_sec = 1e11; // 100 billion/sec
    let classical_seconds = total_keyspace / guesses_per_sec;
    let quantum_seconds = total_keyspace.sqrt() / guesses_per_sec;
    fn human_time(secs: f64) -> String {
        if secs < 60.0 {
            format!("{:.2} seconds", secs)
        } else if secs < 3600.0 {
            format!("{:.2} minutes", secs/60.0)
        } else if secs < 86400.0 {
            format!("{:.2} hours", secs/3600.0)
        } else if secs < 31_536_000.0 {
            format!("{:.2} days", secs/86400.0)
        } else {
            format!("{:.2} years", secs/31_536_000.0)
        }
    }
    println!("\n--- Brute-Force Estimate ---");
    println!("Total entropy: {:.2} bits", total_entropy);
    println!("Total keyspace: ~{:.2e}", total_keyspace);
    println!("Classical brute-force: {}", human_time(classical_seconds));
    println!("Quantum brute-force: {}", human_time(quantum_seconds));
    if total_entropy < 60.0 {
        println!("Warning: Your passwords are weak. Consider using longer or more complex passwords.");
    } else if total_entropy < 80.0 {
        println!("Caution: Your passwords are moderate. For strong security, aim for 80+ bits of entropy.");
    } else {
        println!("Good: Your passwords are strong.");
    }
}
