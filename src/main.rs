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

    // Get user confirmation to launch (press any key)
    let mut garbage = String::new();
    stdin()
        .read_line(&mut garbage)
        .expect("Could not get user confirmation to launch app");

    // Run the app
    match CurseController::new() {
        Ok(mut curse_controller) => match curse_controller.start_event_loop() {
            Ok(_) => println!("Goodbye"),
            Err(error) => println!("{}", error.error_message),
        },
        Err(error) => println!("Error starting app: {}", error.error_message),
    }
}
