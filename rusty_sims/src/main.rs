use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

use database::create_tables;
use rusqlite::Connection;
use std::io::{self, stdin, Write};

use crate::{
    auth::{authenticate_manager, register_manager},
    inventory::{add_product, edit_product, remove_product},
    misc::strip_right,
};

mod auth;
mod database;
mod inventory;
mod misc;

// app must retain a manager's instance as well for the sake of the context
struct Application {
    conn: Connection,
    manager_name: String,
}

enum MenuOptions {
    AddProduct,
    DeleteProduct,
    EditProduct,
    ShowInventory,
    Quit,
    Invalid,
}

impl Application {
    fn run(&mut self) {
        // 1. Initialize state: create tables
        match create_tables(&self.conn) {
            Ok(()) => (),
            Err(e) => {
                println!("{}", e);
                return ();
            }
        }

        self.show_intro();
        self.show_login_screen();

        // by this time, we must have the inventory manager's name with us
        // yeah we have it...!

        // Main Menu
        self.show_menu();

        let mut choice = "".to_string();
        print!(">>  ");
        io::stdout()
            .flush()
            .expect("Error flushing out the console.");

        // select user's choice;
        stdin()
            .read_line(&mut choice)
            .expect("Error reading 'choice' from the terminal");
        strip_right(&mut choice);

        // event loop
        let mut exit = false;
        while !exit {
            match self.str_to_menu_option(choice.as_str()) {
                MenuOptions::AddProduct => {
                    match add_product(&self.conn, &self.manager_name) {
                        Ok(true) => println!("Product added to the inventory!"),
                        Ok(false) => println!("Product couldn't be added to the inventory :("),
                        Err(e) => {
                            println!("Error occurred adding the product ... Aborting operations!");
                            println!("Reason: {}", e);
                            exit = true;
                        }
                    };
                }
                MenuOptions::DeleteProduct => {
                    match remove_product(&self.conn, &self.manager_name) {
                        Ok(true) => println!("Product deleted from the inventory!"),
                        Ok(false) => println!("Product couldn't be deleted from the inventory :("),
                        Err(e) => {
                            println!(
                                "Error occurred deleting the product ... Aborting operations!"
                            );
                            println!("Reason: {}", e);
                            exit = true;
                        }
                    };
                }
                MenuOptions::EditProduct => {
                    let mut product = "".to_string();

                    print!("\nEnter the product's name >>  ");
                    stdin()
                        .read_line(&mut product)
                        .expect("Error reading the product's name from console");
                    strip_right(&mut product);

                    match edit_product(&self.conn, product.as_str(), &self.manager_name) {
                        Ok(_) => (),
                        Err(e) => {
                            println!("Error occurred while updating the product's details");
                            println!("Reason: {}", e);
                            exit = true;
                        }
                    }
                }
                MenuOptions::ShowInventory => {}
                MenuOptions::Quit => {
                    exit = true;
                }
                MenuOptions::Invalid => println!("Invalid Choice, try again!"),
            }
        }
    }

    fn show_menu(&self) {
        let mut table = Table::new();

        table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(80)
            .set_header(vec![Cell::new("Main Menu")
                .add_attribute(Attribute::Bold)
                .set_alignment(CellAlignment::Center)
                .fg(Color::Green)])
            .add_row(vec![Cell::new("1.  Add Product   ").fg(Color::Green)])
            .add_row(vec![Cell::new("2.  Delete Product   ").fg(Color::Green)])
            .add_row(vec![Cell::new("3.  Edit Product   ").fg(Color::Green)])
            .add_row(vec![Cell::new("4.  Show Inventory   ").fg(Color::Green)])
            .add_row(vec![Cell::new("q.  Quit RustySIMS   ").fg(Color::Red)]);

        println!("{table}")
    }

    fn str_to_menu_option(&self, input: &str) -> MenuOptions {
        match input.trim() {
            "1" => MenuOptions::AddProduct,
            "2" => MenuOptions::DeleteProduct,
            "3" => MenuOptions::EditProduct,
            "4" => MenuOptions::ShowInventory,
            "5" => MenuOptions::Quit,
            _ => MenuOptions::Invalid,
        }
    }

    fn show_intro(&self) {
        let title = "Welcome to RustySIMS";
        let sub_title = "Your one-stop Solution for Inventory Management";

        misc::print_welcome_msg(title, sub_title, 60);
    }

    fn input_manager_details(&self) -> (String, String) {
        let mut name = String::from("");
        print!("\nEnter your name >>  ");
        io::stdout()
            .flush()
            .expect("Error flushing out the console");
        io::stdin()
            .read_line(&mut name)
            .expect("Error reading 'name' from console");
        strip_right(&mut name);

        let mut pass = String::from("");
        print!("Enter your password >>  ");
        io::stdout()
            .flush()
            .expect("Error flushing out the console");
        io::stdin()
            .read_line(&mut pass)
            .expect("Error reading 'password' from console");
        strip_right(&mut pass);

        (name, pass)
    }

    fn show_login_screen(&mut self) {
        println!("\n\nPlease login or get registered before you can continue:");

        println!("\nEnter '1' to register");
        println!("Enter '2' to login");

        let mut choice = String::from("");
        loop {
            print!("\n>>  ");
            io::stdout()
                .flush()
                .expect("Error flushing out the console");

            io::stdin()
                .read_line(&mut choice)
                .expect("Error reading 'name' from console");
            strip_right(&mut choice);

            match choice.as_str() {
                "1" => {
                    let (name, pass) = self.input_manager_details();

                    match register_manager(&self.conn, name.as_str(), pass.as_str()) {
                        Ok(()) => {
                            self.manager_name = name;

                            println!("\nSUCCESS!\n");
                            break;
                        }
                        Err(e) => println!("Couldn't add manager: {}", e),
                    }
                }
                "2" => {
                    let (name, pass) = self.input_manager_details();
                    match authenticate_manager(&self.conn, name.as_str(), pass.as_str()) {
                        Ok(()) => {
                            self.manager_name = name;

                            println!("\nSUCCESS\n");
                            break;
                        }
                        Err(e) => println!("\nAuth FAILURE: {}", e),
                    }
                }
                _ => {
                    println!("Invalid choice, try again!");
                    choice.clear();
                }
            }
        }
    }
}

fn main() -> Result<(), rusqlite::Error> {
    let mut app = Application {
        conn: Connection::open("../database.db3")?,
        manager_name: "".to_string(),
    };

    app.run();

    Ok(())
    // application event loop
    // loop {
    // 1. Authentication
    // }
}
