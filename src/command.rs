use std::io::stdin;

use colored::*;
use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct Command {
    pub id: i32,
    pub alias: String,
    pub command: String,
}

pub fn add_command(alias: &str, command: &str) -> Result<()> {
    let conn = Connection::open("./data.db3")?;

    conn.execute(
        "CREATE TABLE if not exists command (
        id    INTEGER PRIMARY KEY,
        alias  TEXT NOT NULL UNIQUE,
        command  TEXT NOT NULL
    )",
        (),
    )?;

    println!("{}", command);

    let command_exists = get_command(alias);

    if command_exists.is_none() {
        let new_command = Command {
            id: 1,
            alias: alias.to_string(),
            command: command.to_string(),
        };

        conn.execute(
            "INSERT INTO command (alias, command) VALUES (?1, ?2)",
            (&new_command.alias, &new_command.command),
        )?;

        println!(
            "{} {:?}",
            "new command added".green(),
            new_command.alias.cyan()
        );

        return Ok(());
    } else {
        let mut input = String::new();

        println!("{}", "Command already exists. Overwrite? (Y/N)".yellow());
        stdin()
            .read_line(&mut input)
            .ok()
            .expect("Failed to read line");

        if input.trim() == "Y" {
            conn.execute(
                "UPDATE command SET command = ?1 WHERE alias = ?2",
                (command, alias),
            )?;

            println!("{} {}", "Command updated:".green(), alias.cyan());

            return Ok(());
        } else {
            println!("{}", "Command not updated, exiting program.".red());
            return Ok(());
        }
    }
}

pub fn run_command(alias: &str) -> Result<()> {
    let command = get_command(alias);

    if command.is_none() {
        println!(
            "{} {}, {}",
            "No command found for alias".red(),
            alias.yellow(),
            "You can add one by running add <alias> <command>".red()
        );
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    let command = command.unwrap();
    println!(
        "{} {}",
        "Running command".cyan(),
        &command.command.bright_green()
    );

    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(["/C", &command.command])
            .output()
            .unwrap();
    } else {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(&command.command)
            .spawn()
            .expect("failed to execute process");
    }

    return Ok(());
}

fn get_command(alias: &str) -> Option<Command> {
    let conn = Connection::open("./data.db3").unwrap();

    let mut stmt = conn
        .prepare("SELECT id, alias, command FROM command WHERE alias = ?1")
        .unwrap();

    let command_iter = stmt
        .query_map([alias], |row| {
            Ok(Command {
                id: row.get(0)?,
                alias: row.get(1)?,
                command: row.get(2)?,
            })
        })
        .unwrap();

    for command in command_iter {
        return Some(command.unwrap());
    }

    return None;
}
