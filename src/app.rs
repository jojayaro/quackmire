use crate::popup::FileNamePopup;
use std::error::{self, Error};
use std::fs;
use std::path::PathBuf;

use duckdb::arrow::util::pretty::pretty_format_batches;
use duckdb::{arrow::array::RecordBatch, Connection};
use ratatui::widgets::ScrollbarState;
use ratatui_explorer::FileExplorer;
use tui_textarea::TextArea;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub connection: Connection,
    pub input: String,
    pub formatted_output: String,
    pub error: Option<String>,
    pub show_error_popup: bool,
    pub vertical_scroll: usize,
    pub horizontal_scroll: usize,
    pub vertical_scroll_state: ScrollbarState,
    pub horizontal_scroll_state: ScrollbarState,
    pub textarea: TextArea<'static>,
    pub file_explorer: FileExplorer,
    pub show_save_popup: bool,
    pub save_popup: FileNamePopup,
}

impl App {
    pub fn new() -> AppResult<Self> {
        Ok(Self {
            running: true,
            connection: Connection::open_in_memory()?,
            input: String::new(),
            formatted_output: String::new(),
            error: None,
            show_error_popup: false,
            vertical_scroll: 0,
            horizontal_scroll: 0,
            vertical_scroll_state: ScrollbarState::default(),
            horizontal_scroll_state: ScrollbarState::default(),
            textarea: TextArea::default(),
            file_explorer: FileExplorer::new()?,
            show_save_popup: false,
            save_popup: FileNamePopup::new(),
        })
    }
    pub fn create_table(&mut self) -> Result<(), Box<dyn Error>> {
        let mut stmt = self.connection.prepare(&self.input)?;
        let rbs: Vec<RecordBatch> = stmt.query_arrow([])?.collect();

        if rbs.is_empty() || rbs[0].num_rows() == 0 {
            self.formatted_output = String::new();
            return Ok(());
        }

        // Use Arrow's native formatting for efficiency
        self.formatted_output = pretty_format_batches(&rbs)
            .map_err(|e| Box::new(e) as Box<dyn Error>)?
            .to_string();

        // Reset scroll position on new query
        self.vertical_scroll = 0;
        self.horizontal_scroll = 0;

        Ok(())
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn toggle_error_popup(&mut self) {
        self.show_error_popup = !self.show_error_popup;
    }

    pub fn scroll_vertical(&mut self, amount: isize) {
        self.vertical_scroll = self.vertical_scroll.saturating_add_signed(amount);
        self.vertical_scroll_state = self.vertical_scroll_state.position(self.vertical_scroll);
    }

    pub fn scroll_horizontal(&mut self, amount: isize) {
        self.horizontal_scroll = self.horizontal_scroll.saturating_add_signed(amount);
        self.horizontal_scroll_state = self
            .horizontal_scroll_state
            .position(self.horizontal_scroll);
    }
    pub fn save_to_file(&self, file_name: &str) -> AppResult<()> {
        let path = PathBuf::from(file_name);
        fs::write(path, &self.input)?;
        Ok(())
    }
    pub fn toggle_save_popup(&mut self) {
        self.show_save_popup = !self.show_save_popup;
        if !self.show_save_popup {
            self.save_popup.input.clear();
        }
    }
}
