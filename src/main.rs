// use std::process::Command;

use command::run_command;

pub mod command;

fn main() {
    let pattern = std::env::args().nth(1);

    if pattern == Some(String::from("add")) {
        let alias = std::env::args().nth(2).unwrap();
        // command is everything that comes after the alias:
        let command = std::env::args().skip(3).collect::<Vec<_>>().join(" ");
        let result = command::add_command(&alias, &command);
        println!("result: {:?}", result);
        return;
    }

    let alias = std::env::args().nth(1);

    match alias {
        Some(_) => {}
        None => panic!("No alias provided"),
    }

    let result = run_command(alias.unwrap().as_str());
    println!("result: {:?}", result);
}
