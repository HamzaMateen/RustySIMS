use std::collections::HashMap;
use std::io::stdin;
use std::process;
use ulid;

use bcrypt::{self, hash, verify};

// Store Inventory Management System
#[derive(Debug)]
struct Product {
    name: String,
    description: String,
    price: f64,
    quantity: i64,
}

struct UserInfo {
    name: String,
    password: String,
}

fn print_titles(title: &str, sub_title: &str, sep_width: usize) {
    let mut pad: i32;
    let mut left_pad: String;
    let mut extra_pad: i32;
    let mut right_pad: String;

    let title_line_width = title.chars().count();
    let sub_title_line_width = sub_title.chars().count();

    if sep_width < title_line_width && sep_width < sub_title_line_width {
        println!("{:<sep_width$}", "");
        println!("{}\n{}", title, sub_title);
        println!("{:<sep_width$}", "");

        ();
    };
    if sep_width > title_line_width {
        // padding for title
        pad = (sep_width - title_line_width) as i32 / 2;
        left_pad = " ".repeat(pad as usize);

        // let's see if the length of title string is even or odd
        // and accordingly decide to include the extra separator or not
        extra_pad = if (sep_width - title_line_width) % 2 == 0 {
            1
        } else {
            0
        };

        right_pad = " ".repeat((pad + extra_pad) as usize);

        println!("{:-<width$}", "", width = sep_width);
        println!("{}{}{}", left_pad, title, right_pad);
    };

    if sep_width > sub_title_line_width {
        pad = (sep_width - sub_title_line_width) as i32 / 2;
        left_pad = " ".repeat(pad as usize);

        // let's see if the length of title string is even or odd
        // and accordingly decide to include the extra separator or not
        extra_pad = if (sep_width - sub_title_line_width) % 2 == 0 {
            1
        } else {
            0
        };

        right_pad = " ".repeat((pad + extra_pad) as usize);

        println!("{}{}{}", left_pad, sub_title, right_pad);
        println!("{:-<width$}", "", width = sep_width);
    }
}

fn authenticate_user(name: &String, password: &String, user: &UserInfo) {
    if *name == user.name {
        match verify(password, &user.password) {
            Ok(true) => println!("Authenticated!"),
            Ok(false) => {
                println!("{},{}", password, user.password);
                println!("Wrong credentials. Aborting ...");
                process::abort();
            }
            Err(_) => println!("Credentials validation error. Sorry!"),
        }
    }
    println!("Wrong username. Aborting ...");
}

fn strip_right(text: &mut String) {
    if text.ends_with("\n") || text.ends_with("\r") {
        text.pop();
        strip_right(text);
    }
    ()
}

type Id = ulid::Ulid;
type Inventory = HashMap<Id, Product>;

fn add_product(inventory: &mut Inventory) {
    let mut new_prod = Product {
        name: "".to_string(),
        description: "".to_string(),
        price: 0.0,
        quantity: 0,
    };

    println!("Enter product name: ");
    stdin()
        .read_line(&mut new_prod.name)
        .expect("Error reading product's name!");

    println!("Provide a description: ");
    stdin()
        .read_line(&mut new_prod.description)
        .expect("Error reading product's description");

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

    new_prod.quantity = match quantity_string.parse::<i64>() {
        Ok(value) => value,
        Err(_) => {
            println!("Error parsing product's quantity string");
            new_prod.quantity
        }
    };

    println!("{:?}", new_prod);

    // add the product to the inventory with a unique ID now
    let id: Id = ulid::Ulid::new();

    inventory.insert(id, new_prod);
}

fn print_inventory(inventory: &mut Inventory) {
    for (key, _value) in inventory.into_iter() {
        println!("{}", key);
    }
}

fn main() {
    // 0. Define inventory now
    // inventory should be a map
    let mut inventory: Inventory = HashMap::new();

    // let inventory: Vec<Product> = Vec::new();
    // 1. Define and create a basic interface
    let title = "Welcome to RustySIMS";
    let sub_title = "You one-stop Inventory Management Solution.";

    print_titles(title, sub_title, 60);

    // 2. Authentication
    let uname = String::from("Hamza");
    let upass = String::from("$2b$12$6yGyx16jFRCVd6e4w2drWerJFdjIh098Hn5IRsbZre9AgFNoepCOa");

    let _registered_user = UserInfo {
        name: uname,
        password: upass,
    };

    let mut name: String = String::from("");
    let mut password: String = String::from("");

    println!("\n\nEnter your name ... ");
    stdin().read_line(&mut name).expect("name reading error");
    strip_right(&mut name);

    println!("Enter your password ... ");
    stdin()
        .read_line(&mut password)
        .expect("password reading error");
    strip_right(&mut password);

    println!("{},{}", name, password);

    // authenticate_user(&name, &password, &registered_user);
    // will authenticate the user later

    // 3. Inventory management
    add_product(&mut inventory);
    print_inventory(&mut inventory);
}
