use std::io;

fn main() {
    println!("Simple Rust Calculator 🧮");

    let a = input("Enter first number: ");
    let b = input("Enter second number: ");
    let op = input("Enter operation (+ - * /): ");

    let a: f64 = a.trim().parse().expect("Not a number!");
    let b: f64 = b.trim().parse().expect("Not a number!");

    let result = match op.trim() {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => a / b,
        _ => {
            println!("Unsupported operation.");
            return;
        }
    };

    println!("Result: {}", result);
}

fn input(prompt: &str) -> String {
    use std::io::Write;
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

