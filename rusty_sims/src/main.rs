mod auth;
mod database;
mod inventory;
mod misc;

use rusqlite::Connection;
use std::io::{self, stdin, Write};

use crate::{
    database::create_tables,
    inventory::{add_product, edit_product, remove_product},
    misc::strip_right,
};

fn main() {
    // 1. Print the welcome screen
    let title = "Welcome to RustySIMS";
    let sub_title = "You one-stop Inventory Management Solution.";

    misc::print_welcome_msg(title, sub_title, 60);

    // 2. Authentication

    println!("\n\nDo you want to register or login to the platform?");
    println!("Enter 1 to register yourself");
    println!("Enter 2 to login to the platform");

    let mut login_choice_str: String = String::from("");
    stdin()
        .read_line(&mut login_choice_str)
        .expect("Error reading manager's choice");
    strip_right(&mut login_choice_str);

    println!("\nOk ...");
    let mut name: String = "".to_string();
    let mut password: String = "".to_string();

    print!("\nEnter your name: ");
    io::stdout().flush().expect("Error flushing stdout");

    stdin()
        .read_line(&mut name)
        .expect("Error reading the name from terminal");
    strip_right(&mut name);

    print!("Enter your password: ");
    io::stdout().flush().expect("Error flushing stdout");

    stdin()
        .read_line(&mut password)
        .expect("Error reading the name from terminal");
    strip_right(&mut password);

    // create connection to the database
    let conn: Connection =
        Connection::open("./database.db3").expect("Error connecting to the database");
    create_tables(&conn).expect("Error creating the tables");

    match login_choice_str.as_str() {
        "1" => {
            match auth::create_manager(&conn, name.as_str(), password.as_str()) {
                Ok(_) => {
                    println!("Manager created succesfully!");
                    ()
                }
                Err(e) => println!("{}", e),
            };
        }
        "2" => {
            auth::authenticate_manager(&conn, name.as_str(), password.as_str());
            println!("Manager authenticated succesfully");
            ()
        }
        _ => {
            println!("\nInvalid choice, please reconsider. Aborting ... \n");
            std::process::abort();
        }
    }

    // 3. add a product to the inventory
    // let's do it
    // add_product(&conn, name.as_str());

    // 3.5 edit a product
    // at this time, we have the manager's name
    let product_name = "Love";
    // edit_product(&conn, product_name, "hamza");

    // 4. Remove a product from the inventory
    remove_product(&conn, name.as_str());
}
