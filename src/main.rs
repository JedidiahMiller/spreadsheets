mod cell_context;
mod curse_controller;
mod errors;
mod expressions;
mod grid;
mod lexer;
mod parser;
mod runtime;
mod statements;
mod token;

use std::io::{Write, stdin, stdout};

use curse_controller::CurseController;

fn main() {
    println!("Congratulations, you are launching the best terminal spreadsheet ever created!");
    print!(
        "WARNING: Do not resize the window while using. You will exit using q. (Hit enter to continue)"
    );
    stdout().flush().unwrap();

    let mut garbage = String::new();
    stdin()
        .read_line(&mut garbage)
        .expect("Could not get user confirmation to launch app");

    // Run the app
    let curse_controller = CurseController::new();
    if curse_controller.is_err() {
        println!(
            "Error starting controller: {}",
            curse_controller.err().unwrap().error_message
        );
        return;
    }
    let mut curse_controller = curse_controller.unwrap();
    match curse_controller.start_event_loop() {
        Ok(_) => {
            println!("Goodbye");
        }
        Err(error) => {
            println!("{}", error.error_message);
        }
    }
}
