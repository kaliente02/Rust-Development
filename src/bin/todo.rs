use std::fs::{OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

fn main() {
    println!("📝 Simple To-Do List");

    loop {
        println!("\nChoose an option:");
        println!("1. View tasks");
        println!("2. Add task");
        println!("3. Exit");

        let choice = input("Enter your choice: ");

        match choice.trim() {
            "1" => view_tasks(),
            "2" => add_task(),
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option."),
        }
    }
}

fn input(prompt: &str) -> String {
    use std::io::Write;
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn view_tasks() {
    let file = OpenOptions::new().read(true).create(true).open("todo.txt").unwrap();
    let reader = BufReader::new(file);

    println!("\n📋 Tasks:");
    for (i, line) in reader.lines().enumerate() {
        println!("{}. {}", i + 1, line.unwrap());
    }
}

fn add_task() {
    let task = input("Enter new task: ");
    let mut file = OpenOptions::new().append(true).create(true).open("todo.txt").unwrap();
    writeln!(file, "{}", task.trim()).unwrap();
    println!("✅ Task added.");
}

