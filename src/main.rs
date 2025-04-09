use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};
use std::io::{self, Write};
use rand::{thread_rng, Rng};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Init,
    Add {
        service: String,
        username: String,
    },
    Get {
        service: String,
    },
    Generate {
        #[arg(long, short, default_value_t = 16)]
        length: i32,
    },
}

struct Login {
    service: String,
    username: String,
    password: String,
}

fn db_init(conn: &Connection) -> Result<()> {    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS store(
            id INTEGER PRIMARY KEY,
            service TEXT NOT NULL,
            username TEXT UNIQUE NOT NULL,
            password TEXT NOT NULL
        )",
        [],
    )?;
    println!("] Initialized");
    Ok(())
}

fn add_login(conn: &Connection, service: String, username: String) -> Result<()> {
    print!("] Password: ");

    io::stdout().flush().unwrap();
    let mut password = String::new();
    
    io::stdin().read_line(&mut password).unwrap();
    let password = password.trim().to_string();

    conn.execute(
        "INSERT INTO store (service, username, password) VALUES (?1, ?2, ?3)",
        [&service, &username, &password],
    )?;
    
    println!("] {} login created.", service);
    Ok(())
}

fn get_login(conn: &Connection, service: String) -> Result<Login> {
    let mut stmt = conn.prepare(
        "SELECT service, username, password FROM store WHERE service = ?1"
    )?;
    
    let login = stmt.query_row([&service], |row| {
        Ok(Login {
            service: row.get(0)?,
            username: row.get(1)?,
            password: row.get(2)?,
        })
    })?;
    
    Ok(login)
}

fn generate_password(length: i32) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789\
                            !@#$%^&*()_-+=[]{}|;:,.<>?";
    
    let mut rng = thread_rng();
    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    
    password
}

fn main() {
    let args = Args::parse();
    let conn = Connection::open("store.db").unwrap();

    match args.command {
        Some(Command::Init) => db_init(&conn).unwrap(),
        Some(Command::Add { service, username }) => add_login(&conn, service, username).unwrap(),
        Some(Command::Get { service }) => {
            match get_login(&conn, service) {
                Ok(login) => {
                    println!("] Service: {}", login.service);
                    println!("] Username: {}", login.username);
                    println!("] Password: {}", login.password);
                },
                Err(e) => eprintln!("Error: {}", e),
            }
        },
        Some(Command::Generate { length }) => {
            if length < 8 {
                eprintln!("Password length must be at least 8 characters");
            } else {
                let password = generate_password(length);
                println!("] Your password: {}", password);
            }
        },
        None => {
            println!("No command provided. Available commands:");
            println!("  init      - Initialize the password manager");
            println!("  add       - Add a new password entry");
            println!("  get       - Get a password entry");
            println!("  generate  - Generate a random password");
        },
    }
}