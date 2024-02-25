use crate::misc;
use bcrypt::{hash, DEFAULT_COST};

use rusqlite::{Connection, Result};

pub fn create_product(
    conn: &Connection,
    name: &str,
    description: &str,
    price: f64,
    stock: i32,
    manager_id: i32,
) -> Result<()> {
    conn.execute(
        "INSERT INTO products_inventory (name, description, price, stock, manager_id) VALUES",
        &[
            name,
            description,
            &price.to_string(),
            &stock.to_string(),
            &manager_id.to_string(),
        ],
    )?;

    Ok(())
}

pub fn create_tables() -> Result<()> {
    let conn = Connection::open("./database.db3")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS products_inventory (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL, 
            description TEXT, 
            price REAL NOT NULL, 
            stock INTEGER NOT NULL, 
            manager_id INTEGER NOT NULL, 
            FOREIGN KEY (manager_id) REFERENCES managers(id)
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS managers (
           id INTEGER PRIMARY KEY,
           name TEXT NOT NULL UNIQUE, 
           hashed_password TEXT NOT NULL,
        )",
        [],
    )?;

    Ok(())
}
