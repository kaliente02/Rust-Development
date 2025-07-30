use rand::{thread_rng, Rng};
use std::io;

fn main() {
    println!("🔐 Password Generator 🔐");

    let length = input("Enter password length: ")
        .trim()
        .parse::<usize>()
        .expect("Please enter a valid number!");

    let use_uppercase = confirm("Include uppercase letters? (y/n): ");
    let use_lowercase = confirm("Include lowercase letters? (y/n): ");
    let use_numbers = confirm("Include numbers? (y/n): ");
    let use_symbols = confirm("Include special characters? (y/n): ");

    let mut charset = String::new();

    if use_uppercase {
        charset += "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    }
    if use_lowercase {
        charset += "abcdefghijklmnopqrstuvwxyz";
    }
    if use_numbers {
        charset += "0123456789";
    }
    if use_symbols {
        charset += "!@#$%^&*()-_=+[]{}|;:,.<>?";
    }

    if charset.is_empty() {
        println!("⚠️ No character set selected. Exiting.");
        return;
    }

    let password = generate_password(length, &charset);
    println!("\nGenerated password: 🔑 {}\n", password);
}

fn input(prompt: &str) -> String {
    use std::io::Write;
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn confirm(prompt: &str) -> bool {
    let answer = input(prompt);
    matches!(answer.trim().to_lowercase().as_str(), "y" | "yes")
}

fn generate_password(length: usize, charset: &str) -> String {
    let mut rng = thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect()
}
