mod auth;
mod database;
mod misc;
// use crate::auth;
//
// use std::collections::HashMap;
// use std::io::stdin;
// use std::process;
// use ulid;
//
// // Store Inventory Management System
// #[derive(Debug)]
// struct Product {
//     name: String,
//     description: String,
//     price: f64,
//     quantity: i64,
// }
//

//
// type Id = ulid::Ulid;
// type Inventory = HashMap<Id, Product>;
//
// fn add_product(inventory: &mut Inventory) {
//     let mut new_prod = Product {
//         name: "".to_string(),
//         description: "".to_string(),
//         price: 0.0,
//         quantity: 0,
//     };
//
//     println!("Enter product name: ");
//     stdin()
//         .read_line(&mut new_prod.name)
//         .expect("Error reading product's name!");
//
//     println!("Provide a description: ");
//     stdin()
//         .read_line(&mut new_prod.description)
//         .expect("Error reading product's description");
//
//     println!("What should be the price: ");
//     let mut price_string: String = "".to_string();
//     stdin()
//         .read_line(&mut price_string)
//         .expect("Error reading product's price");
//     strip_right(&mut price_string);
//
//     new_prod.price = match price_string.parse::<f64>() {
//         Ok(value) => value,
//         Err(_) => {
//             println!("Error parsing product's price, sorry.");
//             new_prod.price
//         }
//     };
//
//     println!("How many of these are in stock: ");
//     let mut quantity_string = "".to_string();
//     stdin()
//         .read_line(&mut quantity_string)
//         .expect("Error reading product's quantity");
//     strip_right(&mut quantity_string);
//
//     new_prod.quantity = match quantity_string.parse::<i64>() {
//         Ok(value) => value,
//         Err(_) => {
//             println!("Error parsing product's quantity string");
//             new_prod.quantity
//         }
//     };
//
//     println!("{:?}", new_prod);
//
//     // add the product to the inventory with a unique ID now
//     let id: Id = ulid::Ulid::new();
//
//     inventory.insert(id, new_prod);
// }
//
// fn print_inventory(inventory: &mut Inventory) {
//     for (key, _value) in inventory.iter_mut() {
//         println!("{}", key);
//     }
// }
//
//

fn main() {
    let title = "Welcome to RustySIMS";
    let sub_title = "You one-stop Inventory Management Solution.";

    misc::print_welcome_msg(title, sub_title, 60);

    // // 2. Authentication
    // let uname = String::from("Hamza");
    // let upass = String::from("$2b$12$6yGyx16jFRCVd6e4w2drWerJFdjIh098Hn5IRsbZre9AgFNoepCOa");
    //
    // let _registered_user = UserInfo {
    //     name: uname,
    //     password_hash: upass,
    // };
    //
    // let mut name: String = String::from("");
    // let mut password: String = String::from("");
    //
    // println!("\n\nEnter your name ... ");
    // stdin().read_line(&mut name).expect("name reading error");
    // strip_right(&mut name);
    //
    // println!("Enter your password ... ");
    // stdin()
    //     .read_line(&mut password)
    //     .expect("password reading error");
    // strip_right(&mut password);
    //
    // println!("{},{}", name, password);
    //
    // // authenticate_user(&name, &password, &registered_user);
    // // will authenticate the user later
    //
    // // 3. Inventory management
    // add_product(&mut inventory);
    // print_inventory(&mut inventory);
}
