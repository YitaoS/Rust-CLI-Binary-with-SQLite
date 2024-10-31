use anyhow::{Context, Result as AnyhowResult};
use clap::{Parser, Subcommand};
use log::info;
use rusqlite::{params, Connection};
use simplelog::{Config, LevelFilter, WriteLogger};
use std::fs::OpenOptions;

#[derive(Debug)]
#[allow(dead_code)]
struct User {
    id: i32,
    name: String,
    age: i32,
}

fn create_table(conn: &Connection) -> AnyhowResult<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            age INTEGER
        )",
        [],
    )?;
    info!("Table created successfully or already exists.");
    Ok(())
}

fn insert_user(conn: &Connection, name: &str, age: i32) -> AnyhowResult<()> {
    conn.execute(
        "INSERT INTO users (name, age) VALUES (?1, ?2)",
        params![name, age],
    )?;
    info!("Inserted user: {} with age: {}", name, age);
    Ok(())
}

fn read_users(conn: &Connection) -> AnyhowResult<Vec<User>> {
    let mut stmt = conn.prepare("SELECT id, name, age FROM users")?;
    let user_iter = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        })
    })?;

    let mut users = Vec::new();
    for user in user_iter {
        users.push(user?);
    }
    info!("Fetched {} users from the database.", users.len());
    Ok(users)
}

fn update_user(conn: &Connection, name: &str, new_age: i32) -> AnyhowResult<()> {
    conn.execute(
        "UPDATE users SET age = ?1 WHERE name = ?2",
        params![new_age, name],
    )?;
    info!("Updated age of user: {} to {}", name, new_age);
    Ok(())
}

fn delete_user(conn: &Connection, name: &str) -> AnyhowResult<()> {
    conn.execute("DELETE FROM users WHERE name = ?1", params![name])?;
    info!("Deleted user: {}", name);
    Ok(())
}

// CLI command definitions using `clap`
#[derive(Parser)]
#[command(
    name = "UserDB",
    about = "A CLI for managing users in a SQLite database."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new user
    Create {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        age: i32,
    },
    /// Read all users
    Read,
    /// Update an existing user's age
    Update {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        age: i32,
    },
    /// Delete a user
    Delete {
        #[arg(short, long)]
        name: String,
    },
}

fn main() -> AnyhowResult<()> {
    // Set up logging to file in append mode
    let log_file = OpenOptions::new()
        .create(true)
        .append(true) // Open file in append mode
        .open("log.txt")
        .context("Failed to open log file")?;

    WriteLogger::init(LevelFilter::Info, Config::default(), log_file)?;

    // Parse CLI arguments
    let cli = Cli::parse();

    // Set up database connection
    let db_path = "database1.db";
    let conn = Connection::open(db_path).context("Failed to open database")?;
    create_table(&conn)?;

    // Handle commands
    match cli.command {
        Commands::Create { name, age } => {
            insert_user(&conn, &name, age)?;
            println!("User created: {} with age {}", name, age);
        }
        Commands::Read => {
            let users = read_users(&conn)?;
            for user in users {
                println!("ID: {}, Name: {}, Age: {}", user.id, user.name, user.age);
            }
        }
        Commands::Update { name, age } => {
            update_user(&conn, &name, age)?;
            println!("User updated: {} with new age {}", name, age);
        }
        Commands::Delete { name } => {
            delete_user(&conn, &name)?;
            println!("User deleted: {}", name);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn setup_test_db() -> AnyhowResult<Connection> {
        let conn = Connection::open_in_memory()?;
        create_table(&conn)?;
        Ok(conn)
    }

    #[test]
    fn test_create_table() -> AnyhowResult<()> {
        let conn = setup_test_db()?;
        assert!(create_table(&conn).is_ok());
        Ok(())
    }

    #[test]
    fn test_insert_user() -> AnyhowResult<()> {
        let conn = setup_test_db()?;
        assert!(insert_user(&conn, "Alice", 30).is_ok());
        Ok(())
    }

    #[test]
    fn test_read_users() -> AnyhowResult<()> {
        let conn = setup_test_db()?;
        insert_user(&conn, "Alice", 30)?;
        let users = read_users(&conn)?;
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].name, "Alice");
        assert_eq!(users[0].age, 30);
        Ok(())
    }

    #[test]
    fn test_update_user() -> AnyhowResult<()> {
        let conn = setup_test_db()?;
        insert_user(&conn, "Alice", 30)?;
        update_user(&conn, "Alice", 35)?;
        let users = read_users(&conn)?;
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].age, 35);
        Ok(())
    }

    #[test]
    fn test_delete_user() -> AnyhowResult<()> {
        let conn = setup_test_db()?;
        insert_user(&conn, "Alice", 30)?;
        delete_user(&conn, "Alice")?;
        let users = read_users(&conn)?;
        assert!(users.is_empty());
        Ok(())
    }
}
