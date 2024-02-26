use crate::database;
// fn add_product(inventory: &mut Inventory) {
//     let mut new_prod = Product {
//         name: "".to_string(),
//         description: "".to_string(),
//         price: 0.0,
//         quantity: 0,
//     };
//
//     println!("Enter product's name: ");
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
