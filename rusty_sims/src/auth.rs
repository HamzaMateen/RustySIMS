use crate::misc::{self, strip_right};
use bcrypt::{hash, verify, DEFAULT_COST};
use rusqlite::{params, Connection, Result};

pub struct Manager {
    id: i32,
    name: String,
    hashed_password: String,
}

pub fn authenticate_manager(conn: &Connection, name: &str, password: &str) {
    // get the manager from the database
    let mut statement = conn
        .prepare("SELECT id, name, hashed_password FROM managers WHERE name = ?1")
        .expect("Error preparing the query statement");

    let mut manager_iter = statement
        .query_map(params![name], |row| {
            Ok(Manager {
                id: row.get(0)?,
                name: row.get(1)?,
                hashed_password: row.get(2)?,
            })
        })
        .expect("Error constructing the iterator");

    // used this while loop since the pattern in question is refutable
    while let Some(result) = manager_iter.next() {
        let manager = result.expect("Failed to fetch manager data from table");

        let mut temp_pass = password.to_string();
        strip_right(&mut temp_pass);

        match verify(temp_pass, &manager.hashed_password) {
            Ok(true) => (),
            Ok(false) => {
                println!("Wrong credentials ... aborting!");
                std::process::abort();
            }
            Err(_) => {
                println!("Authentication system has been compromised. Aborting...");
                std::process::abort();
            }
        }
    }
}

pub fn create_manager(conn: &Connection, name: &str, password: &str) -> Result<()> {
    let temp_pass = password.to_string();
    misc::strip_right(&mut temp_pass.to_string());

    // unwrap() causes a panic
    let hashed_pass =
        hash(temp_pass.as_str(), DEFAULT_COST).expect("Error generating the hash of the password");

    // search for given username in the table, if it exists, abort!
    let mut statement = conn
        .prepare("SELECT EXISTS(SELECT 1 FROM managers WHERE name = ?1)")
        .expect("Error during query preparation");

    // extract the value now
    let exists = statement
        .query_row(params![name], |row| row.get(0))
        .unwrap_or(false);

    if exists {
        panic!("Manager already exists!");
    }

    conn.execute(
        "INSERT INTO managers (name, hashed_password) VALUES (?1, ?2)",
        &[name, &hashed_pass],
    )
    .expect("Error inserting the manager");

    Ok(())
}
