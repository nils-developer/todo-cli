use std::env;
use std::fs;
use std::io::{self, Write};

enum Command {
    Add(String),
    List,
    Remove(usize),
}

const FILENAME: &str = "todo.txt";

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = match parse_command(&args) {
        Ok(cmd) => cmd,
        Err(msg) => {
            eprintln!("{msg}");
            return;
        }
    };

    if let Err(err) = execute(command) {
        eprintln!("Error: {}", err)
    }
}

fn parse_command(args: &[String]) -> Result<Command, String> {
    if args.len() < 2 {
        return Err("Usage: todo-cli <add|list> [task]".into());
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                Err("Usage: todo-cli add \"Aufgabe\"".into())
            } else {
                Ok(Command::Add(args[2].clone()))
            }
        }
        "list" => Ok(Command::List),
        "remove" => {
            if args.len() < 3 {
                return Err("Usage: todo-cli remove <number>".into())
            }

            let index = args[2]
                .parse::<usize>()
                .map_err(|_| "Number must be valid!")?;

            Ok(Command::Remove(index))
        }
        _ => Err("Unknown command. Use 'add' or 'list'.".into()),
    }
}

fn execute(command: Command) -> io::Result<()> {
    match command {
        Command::Add(task) => append_to_file(FILENAME, &task),
        Command::List => list_tasks(FILENAME),
        Command::Remove(index) => remove_task(FILENAME, index),
    }
}

fn list_tasks(path: &str) -> io::Result<()> {
    let contents = fs::read_to_string(path)?;

    for (index, line) in contents.lines().enumerate() {
        println!("{}: {}", index + 1, line)
    }
    Ok(())
}

fn append_to_file(path: &str, text: &str) -> io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    writeln!(file, "{}", text)?;
    Ok(())
}

fn remove_task(path: &str, index: usize) -> io::Result<()> {
    let contents = fs::read_to_string(path)?;
    let mut lines: Vec<String> = contents.lines().map(String::from).collect();

    if index == 0 || index > lines.len() {
        eprintln!("Invalid number!");
        return Ok(());
    }

    let removed = lines.remove(index - 1);

    fs::write(path, lines.join("\n"))?;

    println!("Removed task: {}", removed);
    Ok(())
}
