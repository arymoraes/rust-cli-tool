use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct Command {
    pub id: i32,
    pub alias: String,
    pub command: String,
}

pub fn add_command(alias: &str, command: &str) -> Result<()> {
    let conn = Connection::open("./mydb.db3")?;

    conn.execute(
        "CREATE TABLE if not exists command (
        id    INTEGER PRIMARY KEY,
        alias  TEXT NOT NULL UNIQUE,
        command  TEXT NOT NULL
    )",
        (),
    )?;

    let new_command = Command {
        id: 1,
        alias: alias.to_string(),
        command: command.to_string(),
    };
    conn.execute(
        "INSERT INTO command (alias, command) VALUES (?1, ?2)",
        (&new_command.alias, &new_command.command),
    )?;

    println!("New command: {:?}", new_command.alias);

    Ok(())
}

pub fn run_command(alias: &str) -> Result<()> {
    let conn = Connection::open("./mydb.db3")?;

    let query = format!(
        "SELECT id, alias, command FROM command WHERE alias = '{}'",
        alias
    );

    let mut stmt = conn.prepare(&query)?;

    let mut command_iter = stmt.query_map([], |row| {
        Ok(Command {
            id: row.get(0)?,
            alias: row.get(1)?,
            command: row.get(2)?,
        })
    })?;

    // if we have at least one command run it otherwise panic
    if let Some(command) = command_iter.next() {
        let command = command.unwrap();
        println!("Running command: {:?}", command.command);
        std::process::Command::new("cmd")
            .args(["/C", &command.command])
            .output()
            .unwrap();
        return Ok(());
    }

    panic!("No command found for alias: {}", alias);
}
