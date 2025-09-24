
mod curse_controller;
mod runtime;
mod errors;
mod grid;
mod expressions;
mod token;
mod parser;
mod lexer;
mod statements;
mod cell_context;

use std::io::{stdout, Write, stdin};

use curse_controller::CurseController;

fn main() {

    println!("Congratulations, you are launching the best terminal spreadsheet ever created!");
    print!("WARNING: Do not resize the window while using. You will exit using q. (Hit enter to continue)");
    stdout().flush().unwrap();
    
    let mut garbage = String::new();
    stdin().read_line(&mut garbage).expect("Could not get user confirmation to launch app");

    // Run the app
    let curse_controller = CurseController::new();
    if curse_controller.is_err() {
        println!("Error starting controller: {}", curse_controller.err().unwrap().error_message);
        return
    }
    let mut curse_controller = curse_controller.unwrap();
    match curse_controller.start_event_loop() {
        Ok(_) => {
            println!("Goodbye");
        },
        Err(error) => {
            println!("{}", error.error_message);
        },
    }
}
