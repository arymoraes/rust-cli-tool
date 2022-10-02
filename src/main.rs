use colored::*;
use command::run_command;

pub mod command;

fn main() {
    let pattern = std::env::args().nth(1);

    if pattern == Some(String::from("add")) {
        let alias = std::env::args().nth(2).unwrap();
        // command is everything that comes after the alias:
        let command = std::env::args().skip(3).collect::<Vec<_>>().join(" ");
        command::add_command(&alias, &command).unwrap();
        return;
    }

    let alias = std::env::args().nth(1);

    match alias {
        Some(command_alias) => {
            println!(
                "{} {}",
                "Running command for:".cyan(),
                command_alias.bright_blue()
            );
            let result = run_command(&command_alias.as_str());

            match result {
                Ok(_) => {
                    println!("{}", "Command ran successfully".green());
                }
                Err(_) => {
                    println!("{}", "Exited without executing.".red());
                }
            }
        }
        None => {
            println!("{}", "No alias provided".red());
            return;
        }
    }
}
