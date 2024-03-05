use std::io::{self, Write};

use auth::{authenticate_manager, Manager};
use database::create_tables;
use rusqlite::Connection;

use crate::{auth::register_manager, misc::strip_right};

mod auth;
mod database;
mod inventory;
mod misc;

struct Application {
    conn: Connection,
    // app must retain a manager's instance as well for the sake of the context
}

impl Application {
    fn run(&self) {
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

    fn show_login_screen(&self) {
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
    let app = Application {
        conn: Connection::open("../database.db3")?,
    };

    app.run();

    Ok(())
    // application event loop
    // loop {
    // 1. Authentication
    // }
}
