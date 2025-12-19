/**
 * CLI-TODO
 * Tasks:
 * - Add Task
 * - Show Task
 * - Delete Task
*/

use std::env;
use std::fs;
use std::io::{self, Write};

enum Command {
    Add(String),
    List,
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
        _ => Err("Unknown command. Use 'add' or 'list'.".into()),
    }
}

fn execute(command: Command) -> io::Result<()> {
    match command {
        Command::Add(task) => append_to_file(FILENAME, &task),
        Command::List => list_tasks(FILENAME),
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

