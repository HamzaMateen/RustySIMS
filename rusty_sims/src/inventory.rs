use crate::misc::strip_right;
use rusqlite::{params, Connection, Result, Statement};
use std::io::{stdin, stdout, Write};

// #[derive(Debug)]
pub struct Product {
    name: String,
    description: String,
    price: f64,
    quantity: i32,
    manager_id: i32,
}

// Helper functions
fn get_manager_id(conn: &Connection, name: &str) -> Result<i32> {
    let mut stmnt = conn
        .prepare("SELECT id FROM managers WHERE name = ?1")
        .expect("Error creating query statement");

    stmnt
        .query_row(params![name], |row| Ok(row.get(0)))
        .expect("Fetching id for manager failed")
}

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

fn update_product(conn: &Connection, product: &Product) -> Result<i32> {
    let mut stmnt = match conn.prepare(
        "UPDATE products_inventory SET name = ?1, description = ?2, price = ?3, stock = ?4",
    ) {
        Ok(statement) => statement,
        Err(e) => return Err(e),
    };

    let rows_affected: i32 = match stmnt.execute(params![
        product.name,
        product.description,
        product.price,
        product.quantity
    ]) {
        Ok(rows_affected) => rows_affected as i32,
        Err(e) => return Err(e),
    };
    Ok(rows_affected)
}

fn delete_product(conn: &Connection, product_name: &str, manager_name: &str) -> Result<i32> {
    let manager_id: i32 = match get_manager_id(conn, manager_name) {
        Ok(id) => id,
        Err(e) => return Err(e), // this will propagate the error to the caller
    };

    let mut query_stmnt = match conn
        .prepare("DELETE FROM products_inventory WHERE (name = ?1) AND (manager_id = ?2)")
    {
        Ok(statement) => statement,
        Err(e) => return Err(e),
    };

    let affected_rows_count: i32 = match query_stmnt.execute(params![product_name, manager_id]) {
        Ok(updated_rows_count) => {
            println!(
                "{} row(s) were affected by DELETE operation",
                updated_rows_count
            );
            updated_rows_count as i32
        }
        Err(e) => return Err(e),
    };
    Ok(affected_rows_count)
}

// public api
pub fn edit_product(conn: &Connection, product_name: &str, manager_name: &str) -> Result<i32> {
    let mut updated_product: Product = Product {
        name: String::new(),
        description: String::new(),
        price: 0.0,
        quantity: 0,
        manager_id: 0,
    };

    let manager_id = match get_manager_id(conn, manager_name) {
        Ok(id) => id,
        Err(e) => return Err(e),
    };

    // fetch the product from db
    let mut stmnt = match conn.prepare("SELECT name, description, price, stock, manager_id FROM products_inventory WHERE name = ?1 AND manager_id = ?2") {
        Ok(statement) => statement,
        Err(e) => return Err(e),
    };

    // execute the statement
    stmnt.query_row(params![product_name, manager_id], |row| {
        Ok({
            updated_product.name = row.get(0)?;
            updated_product.description = row.get(1)?;
            updated_product.price = row.get(2)?;
            updated_product.quantity = row.get(3)?;
            updated_product.manager_id = row.get(4)?;
        })
    })?;

    // let's update each and every value from here on
    // we still have the original product with us which we can use to update the values in db

    let mut input = String::new();

    println!("\nUpdating product details. Press <enter> to retain current value:");

    // update name
    print!(
        "\nEnter product's new name? (current: '{}') >>  ",
        updated_product.name
    );
    std::io::stdout()
        .flush()
        .expect("Couldn't flush output console");

    stdin()
        .read_line(&mut input)
        .expect("error reading input from the console");
    strip_right(&mut input);

    if !input.is_empty() {
        updated_product.name = input.clone();
    }
    input.clear();

    // update description
    print!(
        "\nEnter product's new description? (current: {}') >>  ",
        updated_product.description
    );
    std::io::stdout()
        .flush()
        .expect("Couldn't flush output console");

    stdin()
        .read_line(&mut input)
        .expect("error reading input from the console");
    strip_right(&mut input);

    if !input.is_empty() {
        updated_product.description = input.clone();
    }

    // update price
    print!(
        "Enter product's new price? (current: '{}') >> ",
        updated_product.price
    );
    std::io::stdout()
        .flush()
        .expect("Couldn't flush output console");
    stdin()
        .read_line(&mut input)
        .expect("error reading input from the console");
    strip_right(&mut input);

    // parse the value for new price
    if !input.is_empty() {
        updated_product.price = match input.parse() {
            Ok(new_price) => new_price,
            Err(_) => {
                println!("Couldn't parse product's new 'price', retaining current price.");
                updated_product.price
            }
        }
    }

    // update stock value
    print!(
        "Enter product's new quantity available in stock? (current: '{}') |> ",
        updated_product.quantity
    );
    std::io::stdout()
        .flush()
        .expect("Couldn't flush output console");

    stdin()
        .read_line(&mut input)
        .expect("error reading input from the console");
    strip_right(&mut input);

    // parse the value for new quanity
    if !input.is_empty() {
        updated_product.quantity = match input.parse() {
            Ok(new_quantity) => new_quantity,
            Err(_) => {
                println!("Couldn't parse product's new 'stock' value, retaining current quantity.");
                updated_product.quantity
            }
        }
    }
    // no need to update the manager's value, security risk

    // update the database
    match update_product(conn, &updated_product) {
        Ok(rows_affected) => {
            if rows_affected == 0 {
                println!(
                    "Update FAILURE: The product {} is not present in the inventory.",
                    product_name
                );
                Ok(rows_affected)
            } else {
                println!(
                    "Update SUCCESS: Product {}'s details have been updated.",
                    product_name
                );
                Ok(rows_affected)
            }
        }
        Err(e) => Err(e),
    }
}

pub fn remove_product(conn: &Connection, manager_name: &str) -> Result<bool> {
    let mut product_name: String = "".to_string();

    print!("\nWhich product do you want to remove from the inventory? >>  ");
    match std::io::stdout().flush() {
        Ok(()) => (),
        Err(e) => {
            println!("Error flushing out the console: {}", e);
            return Ok(false);
        } // this error shouldn't stop the flow of execution since
          // it doesn't affect the logic in any way
    }

    match stdin().read_line(&mut product_name) {
        Ok(_bytes_read) => (),
        Err(e) => {
            println!("couldn't read bytes from the console: {}", e);
            return Ok(false);
        }
    }
    strip_right(&mut product_name);

    // remove the product using the private api
    match delete_product(conn, product_name.as_str(), manager_name) {
        Ok(value) => {
            if value == 0 {
                println!(
                    "The product {} is not present in the inventory",
                    product_name
                );
                return Ok(false);
            } else {
                println!("The product {} has been taken off the shelf", product_name);
                return Ok(true);
            }
        }
        Err(e) => {
            println!("Error removing the product from inventory: {}", e);
            return Err(e);
        }
    };
}

pub fn add_product(conn: &Connection, manager_name: &str) -> Result<bool> {
    let mut new_prod = Product {
        name: "".to_string(),
        description: "".to_string(),
        price: 0.0,
        quantity: 0,
        manager_id: 0,
    };

    print!("\nEnter product's name >>  ");
    stdout().flush().expect("Error flushing out the console");

    stdin()
        .read_line(&mut new_prod.name)
        .expect("Error reading product's name!");
    strip_right(&mut new_prod.name);

    print!("Provide a description >>  ");
    stdout().flush().expect("Error flusing out the console");

    stdin()
        .read_line(&mut new_prod.description)
        .expect("Error reading product's description");
    strip_right(&mut new_prod.description);

    print!("\nWhat should be the price >>  ");
    stdout().flush().expect("Error flusing out the console");

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

    print!("\nHow many of these are in stock >>  ");
    stdout().flush().expect("Error flusing out the console");

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
        Ok(()) => Ok(true),
        Err(e) => {
            println!("Error adding the product to the database: {}", e);
            return Ok(false);
        }
    }
}
