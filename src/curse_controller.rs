use pancurses::{
    A_BOLD, ACS_HLINE, ACS_LLCORNER, ACS_LRCORNER, ACS_PLUS, ACS_ULCORNER, ACS_URCORNER, ACS_VLINE,
};
use pancurses::{Window, curs_set, endwin, initscr, noecho};

use crate::cell_context::CellContext;
use crate::curse_controller::OperationMode::{Edit, Normal};
use crate::errors::SourceCodeError;
use crate::errors::{Error, ErrorType::*};
use crate::expressions::Expression;
use crate::grid::Cell;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::runtime::Runtime;
use crate::token::Token;

const GAP_SIZE: i32 = 15;
const SOURCE_CODE_WINDOW: i32 = 8;
const MODAL_HEIGHT: i32 = 7;
const TERMINAL_MIN_WIDTH: i32 = 24;
const TERMINAL_MIN_HEIGHT: i32 = 19;

const TAGLINE: &str = "The best Vim motions TUI spreadsheet ever created";
const CONTINUE_PROMPT: &str = "Press Enter to continue";

// "SPREADSHEETS" banner.
const BANNER: [&str; 8] = [
    " .d8888b.  8888888b.  8888888b.  8888888888        d8888 8888888b.   .d8888b.  888    888 8888888888 8888888888 88888888888 .d8888b.  ",
    "d88P  Y88b 888   Y88b 888   Y88b 888              d88888 888  \"Y88b d88P  Y88b 888    888 888        888            888    d88P  Y88b ",
    "Y88b.      888    888 888    888 888             d88P888 888    888 Y88b.      888    888 888        888            888    Y88b.      ",
    " \"Y888b.   888   d88P 888   d88P 8888888        d88P 888 888    888  \"Y888b.   8888888888 8888888    8888888        888     \"Y888b.   ",
    "    \"Y88b. 8888888P\"  8888888P\"  888           d88P  888 888    888     \"Y88b. 888    888 888        888            888        \"Y88b. ",
    "      \"888 888        888 T88b   888          d88P   888 888    888       \"888 888    888 888        888            888          \"888 ",
    "Y88b  d88P 888        888  T88b  888         d8888888888 888  .d88P Y88b  d88P 888    888 888        888            888    Y88b  d88P ",
    " \"Y8888P\"  888        888   T88b 8888888888 d88P     888 8888888P\"   \"Y8888P\"  888    888 8888888888 8888888888     888     \"Y8888P\"  ",
];
const FALLBACK_TITLE: &str = "SPREADSHEETS";

fn add_centered_line(window: &Window, y: i32, max_x: i32, text: &str) {
    let x = ((max_x - text.len() as i32) / 2).max(0);
    window.mvaddstr(y, x, text);
}

enum OperationMode {
    Normal,
    Edit,
    Command,
    Modal,
}

pub struct CurseController {
    master_window: Window,
    source_code_window: Window,
    grid_window: Window,
    grid_size: (usize, usize),
    cursor_position: (i32, i32),
    runtime: Box<Runtime>,
    operation_mode: OperationMode,
    command_buffer: Option<String>,
    new_source_code: String,
    last_error: Option<SourceCodeError>,
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

        let grid_height: i32 = y - SOURCE_CODE_WINDOW;

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
        let last_error = None;

        Ok(CurseController {
            master_window,
            source_code_window,
            grid_window,
            grid_size,
            cursor_position,
            runtime,
            operation_mode,
            command_buffer,
            new_source_code,
            last_error,
        })
    }

    pub fn show_start_screen(&self) -> Result<(), Error> {
        let window = &self.master_window;
        window.clear();

        let (max_y, max_x) = window.get_max_yx();

        let banner_width = BANNER.iter().map(|line| line.len()).max().unwrap_or(0) as i32;
        let use_banner = banner_width <= max_x;
        let banner_height = if use_banner { BANNER.len() as i32 } else { 1 };

        // banner + gap + tagline + larger gap + prompt
        let block_height = banner_height + 9;
        let mut y = ((max_y - block_height) / 2).max(0);

        if use_banner {
            for line in BANNER.iter() {
                add_centered_line(window, y, max_x, line);
                y += 1;
            }
        } else {
            window.attron(A_BOLD);
            add_centered_line(window, y, max_x, FALLBACK_TITLE);
            window.attroff(A_BOLD);
            y += 1;
        }

        y += 2;
        add_centered_line(window, y, max_x, TAGLINE);
        y += 5;
        add_centered_line(window, y, max_x, CONTINUE_PROMPT);

        window.refresh();

        loop {
            match window.getch() {
                // Enter key
                Some(pancurses::Input::Character('\n')) => break,
                // Window resized
                Some(pancurses::Input::KeyResize) => {
                    Self::exit();
                    return Err(Error {
                        error_type: UserError,
                        error_message: "Cannot resize window while starting up".to_string(),
                    });
                }
                _ => continue,
            }
        }

        Ok(())
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
                    // Delete key
                    pancurses::Input::Character('\u{7f}') => {
                        self.new_source_code.pop();
                    }
                    // Escape key
                    pancurses::Input::Character('\u{1b}') => {
                        if let Err(error) = self.save_new_source() {
                            self.last_error = Some(error);
                        }
                        self.operation_mode = OperationMode::Normal;
                    }
                    // Return key
                    pancurses::Input::Character('\n') => {
                        if let Err(error) = self.save_new_source() {
                            self.last_error = Some(error);
                        }
                        self.operation_mode = OperationMode::Normal;
                    }
                    pancurses::Input::Character(c) => {
                        self.new_source_code.push(c);
                    }
                    _ => {}
                },
                OperationMode::Command => match key {
                    // Enter key
                    pancurses::Input::Character('\n') => {
                        match self.command_buffer.as_deref() {
                            Some("q") => {
                                Self::exit();
                                return Ok(());
                            }
                            Some("c") => {
                                self.operation_mode = OperationMode::Modal;
                            }
                            _ => {
                                self.operation_mode = OperationMode::Normal;
                            }
                        }

                        self.command_buffer = None;
                    }
                    // Delete key
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
                OperationMode::Modal => match key {
                    // Escape key
                    pancurses::Input::Character('\u{1b}') => {
                        self.operation_mode = OperationMode::Normal;
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
        let working_source_code = String::from(self.new_source_code.trim());
        if working_source_code.len() == 0 {
            return Ok(());
        }

        // Cells starting with = hold an expression, everything else is a
        // plain string literal
        let code = match working_source_code.strip_prefix('=') {
            Some(expression_source) => {
                let tokens = Lexer::lex(&expression_source.to_string())?;
                Parser::parse_code(tokens)?
            }
            None => Expression::String {
                source_token: Token::default(),
                value: working_source_code.clone(),
            },
        };

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
        self.last_error = None;

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

        // Error modal
        if let OperationMode::Modal = self.operation_mode {
            self.render_modal();
        }
    }

    fn render_source_code(&self) {
        let lines = self.new_source_code.lines();
        for (i, line) in lines.enumerate() {
            self.source_code_window
                .mvaddstr(1 + i as i32, 1, format!(" {}", line));
        }
    }

    fn render_modal(&self) {
        let (max_y, max_x) = self.master_window.get_max_yx();
        let modal_width = (max_x * 2 / 3).max(20);
        let modal_height = MODAL_HEIGHT;
        let start_x = (max_x - modal_width) / 2;
        let start_y = (max_y - modal_height) / 2;

        // Clear the modal's area
        let blank_line = " ".repeat(modal_width as usize);
        for y in start_y..(start_y + modal_height) {
            self.master_window
                .mvaddnstr(y, start_x, &blank_line, modal_width);
        }

        // Border
        for x in (start_x + 1)..(start_x + modal_width - 1) {
            self.master_window.mvaddch(start_y, x, ACS_HLINE());
            self.master_window
                .mvaddch(start_y + modal_height - 1, x, ACS_HLINE());
        }
        for y in (start_y + 1)..(start_y + modal_height - 1) {
            self.master_window.mvaddch(y, start_x, ACS_VLINE());
            self.master_window
                .mvaddch(y, start_x + modal_width - 1, ACS_VLINE());
        }
        self.master_window.mvaddch(start_y, start_x, ACS_ULCORNER());
        self.master_window
            .mvaddch(start_y, start_x + modal_width - 1, ACS_URCORNER());
        self.master_window
            .mvaddch(start_y + modal_height - 1, start_x, ACS_LLCORNER());
        self.master_window.mvaddch(
            start_y + modal_height - 1,
            start_x + modal_width - 1,
            ACS_LRCORNER(),
        );

        self.master_window
            .mvaddstr(start_y + 1, start_x + 2, "Cell Error");

        let close_hint = "Esc to close";
        let hint_x = start_x + modal_width - 2 - close_hint.len() as i32;
        self.master_window.mvaddstr(start_y + 1, hint_x, close_hint);

        let message = match &self.last_error {
            Some(error) => error.error_message.clone(),
            None => "No error for this cell.".to_string(),
        };
        self.master_window
            .mvaddnstr(start_y + 3, start_x + 2, message, modal_width - 4);
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
