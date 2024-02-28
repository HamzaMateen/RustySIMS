use rusqlite::{Connection, Result};

pub fn create_tables(conn: &Connection) -> Result<()> {
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
    )
    .expect("Error creating products_inventory table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS managers (
           id INTEGER PRIMARY KEY,
           name TEXT NOT NULL UNIQUE, 
           hashed_password TEXT NOT NULL
        )",
        [],
    )
    .expect("Error creating 'managers' table");

    Ok(())
}
