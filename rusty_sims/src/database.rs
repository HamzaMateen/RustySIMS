use rusqlite::{params, Connection, Error, Result};

pub fn create_product(
    conn: &Connection,
    name: &str,
    description: &str,
    price: f64,
    stock: i32,
    manager_id: i32,
) -> Result<()> {
    conn.execute(
        "INSERT INTO products_inventory (name, description, price, stock, manager_id) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            name,
            description,
            &price.to_string(),
            &stock.to_string(),
            &manager_id.to_string(),
        ],
    )
    .expect("Error inserting the product to the table");

    Ok(())
}

pub fn get_manager_id(conn: &Connection, name: &str) -> Result<i32> {
    let mut stmnt = conn
        .prepare("SELECT id FROM managers WHERE name = ?1")
        .expect("Error creating query statement");

    stmnt
        .query_row(params![name], |row| Ok(row.get(0)))
        .expect("Fetching id for manager failed")
}

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
