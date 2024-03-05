use bcrypt::{hash, verify, DEFAULT_COST};
use rusqlite::{params, Connection};

use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

use std::error::Error;

#[derive(Debug, Default)]
pub struct Manager {
    pub id: i32,
    pub name: String,
    pub hashed_password: String,
}

impl Manager {
    #![allow(dead_code)]
    pub fn show_manager_details(&self) {
        println!("******** Manager's Data ********\n");

        // let's use comfy-table to beautify this now
        let mut table = Table::new();

        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(50)
            .set_header(vec!["ID", "Name", "Hashed Password"])
            .add_row(vec![
                Cell::new(self.id.to_string().as_str()),
                Cell::new(self.name.as_str()),
                Cell::new(self.hashed_password.as_str()),
            ]);

        println!("{table}")
    }
}

// Dynamic dispatch used for error propagation
pub fn authenticate_manager(
    conn: &Connection,
    name: &str,
    password: &str,
) -> Result<(), Box<dyn Error>> {
    // get manager data from the database
    let mut statement =
        conn.prepare("SELECT id, name, hashed_password FROM managers WHERE name = ?1")?;

    let mut manager_iter = statement.query_map(params![name], |row| {
        Ok(Manager {
            id: row.get(0)?,
            name: row.get(1)?,
            hashed_password: row.get(2)?,
        })
    })?;

    // used this while loop since the pattern in question is refutable
    while let Some(result) = manager_iter.next() {
        let manager = result?;

        let temp_pass = password.to_string();
        match verify(temp_pass, &manager.hashed_password) {
            Ok(true) => return Ok(()),
            Ok(false) => return Err(Box::<dyn Error>::from("Wrong credentials... try again")),
            Err(_) => return Err(Box::<dyn Error>::from("Has verification error")),
        }
    }
    Err(Box::<dyn Error>::from("Wrong credentials... try again"))
}

pub fn register_manager(conn: &Connection, name: &str, password: &str) -> rusqlite::Result<()> {
    // () => correct result
    let temp_pass = password.to_string();

    // unwrap() causes a panic
    let hashed_pass =
        hash(temp_pass.as_str(), DEFAULT_COST).expect("Error generating the hash of the password");

    // search for given username in the table, if it exists, abort!
    let mut statement = conn.prepare("SELECT EXISTS(SELECT 1 FROM managers WHERE name = ?1)")?;

    // extract the value now
    let exists: bool = statement
        .query_row(params![name], |row| {
            let exists: i32 = row.get::<_, i32>(0)?;
            Ok(exists == 1)
        })
        .unwrap_or(false);

    if exists {
        // return () only, what that user wants done has already been done
        return Ok(());
    }

    conn.execute(
        "INSERT INTO managers (name, hashed_password) VALUES (?1, ?2)",
        &[name, &hashed_pass],
    )?;

    Ok(())
}
