use crate::misc::strip_right;
use rusqlite::{params, Connection, Result};
use std::io::{stdin, Write};

#[derive(Debug)]
pub struct Product {
    name: String,
    description: String,
    price: f64,
    quantity: i32,
    manager_id: i32,
}

fn get_manager_id(conn: &Connection, name: &str) -> Result<i32> {
    let mut stmnt = conn
        .prepare("SELECT id FROM managers WHERE name = ?1")
        .expect("Error creating query statement");

    stmnt
        .query_row(params![name], |row| Ok(row.get(0)))
        .expect("Fetching id for manager failed")
}

// Helper functions
fn create_product(conn: &Connection, product: &mut Product, manager_name: &str) -> Result<()> {
    let manager_id = get_manager_id(conn, manager_name).unwrap_or_else(|e| {
        println!("Error getting manager ID: {}", e);
        std::process::abort();
    });

    product.manager_id = manager_id;

    conn.execute(
        "INSERT INTO products_inventory (name, description, price, stock, manager_id) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            &product.name,
            &product.description,
            product.price,
            product.quantity,
            manager_id,
        ]
    )
    .expect("Error inserting the product to the table");

    Ok(())
}

fn delete_product(conn: &Connection, product_name: &str, manager_name: &str) -> Result<()> {
    let manager_id = get_manager_id(conn, manager_name).unwrap_or_else(|e| {
        println!("Error getting manager ID: {}", e);
        std::process::abort();
    });

    let mut query_stmnt = conn
        .prepare("DELETE FROM products_inventory WHERE (name = ?1) AND (manager_id = ?2)")
        .expect("Error preparing the query statemetn");

    query_stmnt
        .execute(params![product_name, manager_id])
        .expect("Error deleting the product from the inventory");

    Ok(())
}

// public api
pub fn remove_product(conn: &Connection, manager_name: &str) {
    let mut product_name: String = "".to_string();

    print!("Which product do you want to remove from the inventory? |> ");
    std::io::stdout()
        .flush()
        .expect("Error flushing the standard output terminal");

    stdin()
        .read_line(&mut product_name)
        .expect("Error reading the product's name from the terminal, aborting!");
    strip_right(&mut product_name);

    // remove the product using the private api
    match delete_product(conn, product_name.as_str(), manager_name) {
        Ok(()) => println!(
            "The product {} has been remove the from the inventory",
            product_name
        ),
        Err(e) => {
            println!("Error removing the product from inventory: {}", e);
            std::process::abort();
        }
    };
}

pub fn add_product(conn: &Connection, manager_name: &str) {
    let mut new_prod = Product {
        name: "".to_string(),
        description: "".to_string(),
        price: 0.0,
        quantity: 0,
        manager_id: 0,
    };

    println!("Enter product's name: ");
    stdin()
        .read_line(&mut new_prod.name)
        .expect("Error reading product's name!");
    strip_right(&mut new_prod.name);

    println!("Provide a description: ");
    stdin()
        .read_line(&mut new_prod.description)
        .expect("Error reading product's description");
    strip_right(&mut new_prod.description);

    println!("What should be the price: ");
    let mut price_string: String = "".to_string();
    stdin()
        .read_line(&mut price_string)
        .expect("Error reading product's price");
    strip_right(&mut price_string);

    new_prod.price = match price_string.parse::<f64>() {
        Ok(value) => value,
        Err(_) => {
            println!("Error parsing product's price, sorry.");
            new_prod.price
        }
    };

    println!("How many of these are in stock: ");
    let mut quantity_string = "".to_string();
    stdin()
        .read_line(&mut quantity_string)
        .expect("Error reading product's quantity");
    strip_right(&mut quantity_string);

    new_prod.quantity = match quantity_string.parse::<i32>() {
        Ok(value) => value,
        Err(_) => {
            println!("Error parsing product's quantity string");
            new_prod.quantity
        }
    };

    // add the product now to the 'products_inventory' table
    match create_product(conn, &mut new_prod, manager_name) {
        Ok(()) => println!("Success adding the product to the database."),
        Err(e) => {
            println!("Failure adding the product to the database: {}", e);
            std::process::abort();
        }
    }
}
