use std::io::stdin;

use bcrypt::{self, hash, verify};

// // Store Inventory Management System
struct Product {
    name: String,
    description: String,
    price: f64,
    quantity: i64,
}

struct user_info {
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

fn authenticate_user(name: &String, password: &String, user: &user_info) {
    // let hash_password = match hash(&password, 12) {
    //     Ok(value) => value,
    //     Err(_) => "hashing error".to_string(),
    // };
    //
    if "$2b$12$6yGyx16jFRCVd6e4w2drWerJFdjIh098Hn5IRsbZre9AgFNoepCOa"
        == "$2b$12$6yGyx16jFRCVd6e4w2drWerJFdjIh098Hn5IRsbZre9AgFNoepCOa"
    {
        println!("they match man!");
    }

    if *name == user.name {
        println!(
            "{},{},{}\n",
            name,
            user.password.as_str(),
            password.as_str()
        );
        let is_authenticated = match verify(password, &user.password.as_str()) {
            Ok(status) => {
                println!("comparison done!");
                println!("{}", status);
                status
            }
            Err(_) => false,
        };

        if !is_authenticated {
            println!("Sorry, wrong credentials. Aborting ...");
        }
        ()
    }
}

fn strip_right(text: &mut String) {
    if text.ends_with("\n") || text.ends_with("\r") {
        text.pop();
        strip_right(text);
    }
    ()
}

fn main() {
    // let inventory: Vec<Product> = Vec::new();
    // 1. Define and create a basic interface
    let title = "Welcome to RustySIMS";
    let sub_title = "You one-stop Inventory Management Solution.";

    print_titles(title, sub_title, 60);

    // 2. Authentication
    let uname = String::from("Hamza");
    let upass = String::from("$2b$12$6yGyx16jFRCVd6e4w2drWerJFdjIh098Hn5IRsbZre9AgFNoepCOa");

    let registered_user = user_info {
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
    authenticate_user(&name, &password, &registered_user);
}
