use pancurses::{
    ACS_HLINE, ACS_LLCORNER, ACS_LRCORNER, ACS_PLUS, ACS_ULCORNER, ACS_URCORNER, ACS_VLINE,
};
use pancurses::{Window, curs_set, endwin, initscr, noecho};

use crate::cell_context::CellContext;
use crate::curse_controller::OperationMode::{Edit, Normal};
use crate::errors::SourceCodeError;
use crate::errors::{Error, ErrorType::*};
use crate::grid::Cell;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::runtime::Runtime;
use crate::token::TokenType;

const GAP_SIZE: i32 = 15;
const SOURCE_CODE_WINDOW: i32 = 8;
const FEEDBACK_WINDOW: i32 = 6;
const TERMINAL_MIN_WIDTH: i32 = 24;
const TERMINAL_MIN_HEIGHT: i32 = 19;

enum OperationMode {
    Normal,
    Edit,
    Command,
}

pub struct CurseController {
    master_window: Window,
    source_code_window: Window,
    grid_window: Window,
    feedback_window: Window,
    grid_size: (usize, usize),
    cursor_position: (i32, i32),
    runtime: Box<Runtime>,
    operation_mode: OperationMode,
    command_buffer: Option<String>,
    new_source_code: String,
    feedback: (String, String, String),
}

impl CurseController {
    pub fn new() -> Result<CurseController, Error> {
        let master_window = initscr();
        noecho();
        curs_set(0);
        master_window.keypad(true);

        if master_window.get_max_x() < TERMINAL_MIN_WIDTH
            || master_window.get_max_y() < TERMINAL_MIN_HEIGHT
        {
            Self::exit();
            eprintln!(
                "Please resize your terminal to at least {}x{} (currently {}x{})",
                TERMINAL_MIN_WIDTH,
                TERMINAL_MIN_HEIGHT,
                master_window.get_max_x(),
                master_window.get_max_y()
            );
            std::process::exit(1);
        }

        let (y, x) = master_window.get_max_yx();

        let grid_height: i32 = y - SOURCE_CODE_WINDOW - FEEDBACK_WINDOW;

        // Source code window
        let source_code_window: Window;
        match master_window.subwin(SOURCE_CODE_WINDOW, x, 0, 0) {
            Ok(window) => source_code_window = window,
            Err(_) => {
                return Err(Error {
                    error_type: UnexpectedError,
                    error_message: "Couldn't create source code window".to_string(),
                });
            }
        }

        // Grid window
        let grid_window: Window;
        match master_window.subwin(grid_height, x, SOURCE_CODE_WINDOW, 0) {
            Ok(window) => grid_window = window,
            Err(_) => {
                return Err(Error {
                    error_type: UnexpectedError,
                    error_message: "Couldn't create grid window".to_string(),
                });
            }
        }

        // Data window
        let data_window: Window;
        match master_window.subwin(FEEDBACK_WINDOW, x, SOURCE_CODE_WINDOW + grid_height, 0) {
            Ok(window) => data_window = window,
            Err(_) => {
                return Err(Error {
                    error_type: UnexpectedError,
                    error_message: "Couldn't create data window".to_string(),
                });
            }
        }

        // Grid size
        let width = grid_window.get_max_x() - (grid_window.get_max_x() % GAP_SIZE);
        let width = ((width / GAP_SIZE) - 1) as usize;
        let height = ((grid_height - 1) / 2) as usize;
        let grid_size = (width, height);

        // Default cursor position
        let cursor_position = (0, 0);

        // Runtime
        let runtime = Box::new(Runtime::new(grid_size));

        // Other defaults
        let operation_mode = Normal;
        let command_buffer = None;
        let new_source_code = String::from("");
        let feedback = (String::from(""), String::from(""), String::from(""));

        Ok(CurseController {
            master_window,
            source_code_window,
            grid_window,
            feedback_window: data_window,
            grid_size,
            cursor_position,
            runtime,
            operation_mode,
            command_buffer,
            new_source_code,
            feedback,
        })
    }

    pub fn start_event_loop(&mut self) -> Result<(), Error> {
        loop {
            self.render();

            // Get user input
            let key = self.master_window.getch();
            if key.is_none() {
                continue;
            };
            let key = key.unwrap();

            if (key == pancurses::Input::KeyResize) {
                // Window resize
                Self::exit();
                return Err(Error {
                    error_type: UserError,
                    error_message: "Cannot resize window while running".to_string(),
                });
            }

            match self.operation_mode {
                OperationMode::Normal => match key {
                    pancurses::Input::KeyUp | pancurses::Input::Character('k') => {
                        if self.cursor_position.1 > 0 {
                            self.cursor_position.1 -= 1;
                            self.update_selected_cell();
                        };
                    }
                    pancurses::Input::KeyRight | pancurses::Input::Character('l') => {
                        if self.cursor_position.0 < self.grid_size.0 as i32 - 1 {
                            self.cursor_position.0 += 1;
                            self.update_selected_cell();
                        };
                    }
                    pancurses::Input::KeyDown | pancurses::Input::Character('j') => {
                        if self.cursor_position.1 < self.grid_size.1 as i32 - 1 {
                            self.cursor_position.1 += 1;
                            self.update_selected_cell();
                        };
                    }
                    pancurses::Input::KeyLeft | pancurses::Input::Character('h') => {
                        if self.cursor_position.0 > 0 {
                            self.cursor_position.0 -= 1;
                            self.update_selected_cell();
                        };
                    }
                    pancurses::Input::Character('i') => {
                        self.operation_mode = OperationMode::Edit;
                    }
                    pancurses::Input::Character(':') => {
                        self.command_buffer = Some("".to_string());
                        self.operation_mode = OperationMode::Command;
                    }
                    _ => {}
                },
                OperationMode::Edit => match key {
                    pancurses::Input::Character('\u{7f}') => {
                        self.new_source_code.pop();
                    }
                    pancurses::Input::Character('\u{1b}') => {
                        // Todo: handler source code errors
                        match self.save_new_source() {
                            Ok(_) => {}
                            Err(_) => {}
                        }
                        self.operation_mode = OperationMode::Normal;
                    }
                    // Todo: handle source code errors
                    pancurses::Input::Character('\n') => match self.save_new_source() {
                        Ok(_) => {}
                        Err(_) => {}
                    },
                    pancurses::Input::Character(c) => {
                        self.new_source_code.push(c);
                    }
                    _ => {}
                },
                OperationMode::Command => match key {
                    pancurses::Input::Character('\n') => {
                        match self.command_buffer.as_deref() {
                            Some("q") => {
                                Self::exit();
                                return Ok(());
                            }
                            _ => {}
                        }

                        self.command_buffer = None;
                        self.operation_mode = OperationMode::Normal;
                    }
                    pancurses::Input::Character('\u{7f}') => {
                        if let Some(buffer) = &mut self.command_buffer {
                            buffer.pop();
                        }
                    }
                    pancurses::Input::Character(c) => {
                        if let Some(buffer) = &mut self.command_buffer {
                            buffer.push(c);
                        }
                    }
                    _ => {}
                },
            }
        }
    }

    pub fn exit() {
        endwin();
    }

    fn update_selected_cell(&mut self) {
        let cell = self
            .runtime
            .get_cell(self.cursor_position.0, self.cursor_position.1);
        let cell = cell.unwrap(); // Not sure why this would error. Everything should be checked earlier

        self.new_source_code = cell.source_code.clone();
    }

    fn save_new_source(&mut self) -> Result<(), SourceCodeError> {
        // Clean source code
        let mut working_source_code = String::from(self.new_source_code.trim());
        if working_source_code.len() == 0 {
            return Ok(());
        }

        // Check if starts with '='; if not, turn it into a return statement
        let mut should_be_primitive = true;
        if working_source_code.starts_with('=') {
            working_source_code.remove(0);
            should_be_primitive = false;
        }

        // Create tokens
        // Turn into a return statement if it is a simple primitive
        if should_be_primitive {
            working_source_code = format!("return {};", working_source_code);
        }

        let mut tokens = Lexer::lex(&working_source_code);
        if tokens.is_err() {
            if !should_be_primitive {
                return Err(tokens.err().unwrap());
            }
            tokens = Lexer::lex(&format!("\"{}\"", working_source_code));
        }
        let mut tokens = tokens.unwrap();

        // Check for primitive, cast to string if it should be primitive but is not
        let mut is_primitive = false;
        if tokens.len() == 3 {
            match tokens.get(1).unwrap().token_type {
                TokenType::Integer | TokenType::Float | TokenType::Boolean | TokenType::String => {
                    is_primitive = true
                }
                _ => {}
            };
        }
        // Handle negative numbers
        if tokens.len() == 4 {
            match tokens.get(2).unwrap().token_type {
                TokenType::Negation => match tokens.get(1).unwrap().token_type {
                    TokenType::Integer | TokenType::Float => is_primitive = true,
                    _ => {}
                },
                _ => {}
            };
        }
        // Cast to string
        if should_be_primitive && !is_primitive {
            // Trim the artificial statement stuff
            working_source_code = working_source_code[7..working_source_code.len() - 1].to_string();
            tokens = Lexer::lex(&format!("return \"{}\";", working_source_code)).unwrap();
        }

        // Check code
        let code = Parser::parse_code(tokens);
        if code.is_err() {
            return Err(code.err().unwrap());
        }
        let code = code.unwrap();

        // Evaluate primitive
        let primitive = CellContext::evaluate_with_context(&self.runtime, &code);
        if primitive.is_err() {
            let message = primitive.err().unwrap();
            return Err(SourceCodeError {
                location: vec![0],
                error_message: format!("Expression failed to evaluate: {}", message),
            });
        }
        let primative = Box::new(primitive.unwrap());

        // Create cells
        let backup_cell = *self
            .runtime
            .get_cell(self.cursor_position.0, self.cursor_position.1)
            .unwrap(); // I trust this
        let cell = Cell {
            source_code: self.new_source_code.clone(),
            code: Box::new(code),
            primative: primative.clone(),
        };

        // This should not error because the parameters already are validated
        self.runtime
            .set_cell(self.cursor_position.0, self.cursor_position.1, &cell)
            .unwrap();

        // Check the rest of the cells for conflicts
        match self.update_cell_primitives() {
            Ok(_) => {}
            Err(error) => {
                // Revert to previous
                self.runtime
                    .set_cell(self.cursor_position.0, self.cursor_position.1, &backup_cell)
                    .unwrap();
                return Err(SourceCodeError {
                    location: vec![0],
                    error_message: error,
                });
            }
        }

        // Reset error messages and such
        self.clear_feedback();

        Ok(())
    }

    fn update_cell_primitives(&mut self) -> Result<(), String> {
        for x in 0..self.grid_size.0 {
            for y in 0..self.grid_size.1 {
                let cell = self.runtime.get_cell(x as i32, y as i32);
                let mut cell = *cell.unwrap(); // Not sure why this would error. Everything should be checked earlier

                let evaluation = CellContext::evaluate_with_context(&self.runtime, &cell.code);
                if evaluation.is_err() {
                    return Err(format!(
                        "Error with cell {}, {}: {}",
                        x,
                        y,
                        evaluation.err().unwrap()
                    ));
                }
                cell.primative = Box::new(evaluation.unwrap());
                self.runtime.set_cell(x as i32, y as i32, &cell).unwrap(); // Again, shouldn't error
            }
        }
        Ok(())
    }

    fn render(&self) {
        // Reset
        self.master_window.clear();

        // Source code window
        self.render_source_code();
        self.box_window(&self.source_code_window);
        self.highlight_source_box();

        // Grid
        self.create_grid();
        self.highlight_cell(self.cursor_position.0, self.cursor_position.1);
        self.render_cell_contents();

        // Feedback window
        self.box_window(&self.feedback_window);
        self.render_feedback();
    }

    fn render_source_code(&self) {
        let lines = self.new_source_code.lines();
        for (i, line) in lines.enumerate() {
            self.source_code_window
                .mvaddstr(1 + i as i32, 1, format!(" {}", line));
        }
    }

    fn render_feedback(&self) {
        if FEEDBACK_WINDOW != 6 {
            panic!("Programming is hard. FEEDBACK_WINDOW is expected to be 6.")
        };

        let current_cell_value = self
            .runtime
            .get_cell(self.cursor_position.0, self.cursor_position.1)
            .unwrap()
            .primative;
        let current_cell_value = current_cell_value.serialize().unwrap();

        self.feedback_window
            .mvaddstr(1, 2, format!("Current cell value: {}", current_cell_value));
        self.feedback_window.mvaddstr(2, 2, self.feedback.0.clone());
        self.feedback_window.mvaddstr(3, 2, self.feedback.1.clone());
        self.feedback_window.mvaddstr(4, 2, self.feedback.2.clone());
    }

    fn set_feedback(&mut self, line1: String, line2: String, line3: String) {
        self.feedback = (line1, line2, line3);
    }

    fn clear_feedback(&mut self) {
        self.set_feedback(String::from(""), String::from(""), String::from(""));
    }

    fn show_source_code_error(&mut self, error: SourceCodeError) {
        let working_source_code = String::from(self.new_source_code.trim());
        let is_expression = working_source_code.starts_with('=');
        let working_indices = if is_expression {
            error.location.iter().map(|&x| x + 1).collect::<Vec<_>>()
        } else {
            error.location
        };

        let mut arrows = String::from("");
        for i in 0..self.new_source_code.len() {
            let index = i + if is_expression { 1 } else { 0 };
            if working_indices.contains(&index) {
                arrows.push('^');
            } else {
                arrows.push(' ');
            }
        }

        self.set_feedback(working_source_code, arrows, error.error_message);
    }

    fn render_cell_contents(&self) {
        for x in 0..self.grid_size.0 {
            for y in 0..self.grid_size.1 {
                let cell = self.runtime.get_cell(x as i32, y as i32);
                let cell = cell.unwrap(); // Not sure why this would error. Everything should be checked earlier

                let text = cell.primative.serialize().unwrap(); // Again, shouldn't error

                self.add_cell_text(x as i32, y as i32, text);
            }
        }
    }

    fn box_window(&self, window: &Window) {
        for i in 1..(window.get_max_x() - 1) {
            window.mvaddch(0, i, ACS_HLINE());
            window.mvaddch(window.get_max_y() - 1, i, ACS_HLINE());
        }
        for i in 1..(window.get_max_y() - 1) {
            window.mvaddch(i, 0, ACS_VLINE());
            window.mvaddch(i, window.get_max_x() - 1, ACS_VLINE());
        }

        window.mvaddch(0, 0, ACS_ULCORNER());
        window.mvaddch(0, window.get_max_x() - 1, ACS_URCORNER());
        window.mvaddch(window.get_max_y() - 1, 0, ACS_LLCORNER());
        window.mvaddch(
            window.get_max_y() - 1,
            window.get_max_x() - 1,
            ACS_LRCORNER(),
        );
    }

    fn add_cell_text(&self, x: i32, y: i32, text: String) {
        self.clear_cell(x, y);

        let actual_x = (x + 1) * GAP_SIZE;
        let actual_y = (y + 1) * 2;

        self.grid_window
            .mvaddnstr(actual_y, actual_x, text, GAP_SIZE - 1);
    }

    fn add_centered_cell_text(&self, x: i32, y: i32, text: String) {
        let actual_x = (x + 1) * GAP_SIZE;
        let actual_y = (y + 1) * 2;

        let actual_str = format!("{:^gap$}", text, gap = (GAP_SIZE - 1) as usize);

        self.grid_window
            .mvaddnstr(actual_y, actual_x, actual_str, GAP_SIZE - 1);
    }

    fn clear_cell(&self, x: i32, y: i32) {
        self.add_centered_cell_text(x, y, "".to_string());
    }

    fn highlight_source_box(&self) {
        self.source_code_window.mvaddch(0, 0, 'X');
        self.source_code_window
            .mvaddch(self.source_code_window.get_max_y() - 1, 0, 'X');
        self.source_code_window.mvaddch(
            self.source_code_window.get_max_y() - 1,
            self.source_code_window.get_max_x() - 1,
            'X',
        );
        self.source_code_window
            .mvaddch(0, self.source_code_window.get_max_x() - 1, 'X');
    }

    fn highlight_cell(&self, x: i32, y: i32) {
        let left_corner_x = (x + 1) * GAP_SIZE - 1;
        let left_corner_y = (y + 1) * 2 - 1;

        self.grid_window.mvaddch(left_corner_y, left_corner_x, 'X');
        self.grid_window
            .mvaddch(left_corner_y + 2, left_corner_x, 'X');
        self.grid_window
            .mvaddch(left_corner_y, left_corner_x + GAP_SIZE, 'X');
        self.grid_window
            .mvaddch(left_corner_y + 2, left_corner_x + GAP_SIZE, 'X');
    }

    fn create_grid(&self) {
        let grid_width = self.grid_window.get_max_x() - (self.grid_window.get_max_x() % GAP_SIZE);

        for x in 0..(grid_width) {
            // Width
            for y in 0..self.grid_window.get_max_y() {
                // Height
                if y % 2 == 1 {
                    if x % GAP_SIZE == GAP_SIZE - 1 {
                        self.grid_window.mvaddch(y, x, ACS_PLUS()); // Intersection
                    } else {
                        self.grid_window.mvaddch(y, x, ACS_HLINE()); // Horizontal line
                    }
                } else if x % GAP_SIZE == GAP_SIZE - 1 {
                    self.grid_window.mvaddch(y, x, ACS_VLINE()); // Vertical line
                }
            }
        }

        // Number the rows and columns
        for col in 0..((grid_width / GAP_SIZE) - 1) {
            self.add_centered_cell_text(col, -1, col.to_string());
        }
        for row in 0..(self.grid_window.get_max_y() / 2) {
            self.add_centered_cell_text(-1, row, row.to_string());
        }
    }
}
