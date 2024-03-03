use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::Alignment,
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

mod tui;

// end using tui stuff
mod auth;
mod database;
mod inventory;
mod misc;

use rusqlite::Connection;

use crate::{
    database::create_tables,
    inventory::{add_product, edit_product, remove_product},
    misc::strip_right,
};

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

impl App {
    // runs the app main loop the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.increment_counter(),
            KeyCode::Right => self.decrement_counter(),
            _ => {}
        }
    }

    // business logic
    fn exit(&mut self) {
        self.exit = true;
    }

    fn decrement_counter(&mut self) {
        self.counter += 1;
    }

    fn increment_counter(&mut self) {
        self.counter -= 1;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Welcome to RustySIMS ".bold());
        let instructions = Title::from(Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".bold(),
            " Q ".blue().bold(),
        ]));

        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf)
    }
}

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;

    let mut myapp = App::default();
    myapp.run(&mut terminal)?;

    tui::restore()?;
    Ok(())

    /*

    // 1. Print the welcome screen
    //
    let title = "Welcome to RustySIMS";
    let sub_title = "You one-stop Inventory Management Solution.";

    misc::print_welcome_msg(title, sub_title, 60);

    Ok(())
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
    */
}
