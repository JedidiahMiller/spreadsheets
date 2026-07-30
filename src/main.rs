mod cell_context;
mod curse_controller;
mod errors;
mod expressions;
mod grid;
mod lexer;
mod parser;
mod runtime;
mod token;

use curse_controller::CurseController;

fn main() {
    match CurseController::new() {
        Ok(mut curse_controller) => {
            if let Err(error) = curse_controller.show_start_screen() {
                println!("{}", error.error_message);
                return;
            }
            match curse_controller.start_event_loop() {
                Ok(_) => {}
                Err(error) => println!("{}", error.error_message),
            }
        }
        Err(error) => println!("Error starting app: {}", error.error_message),
    }
}
