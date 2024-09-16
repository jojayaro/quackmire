use crate::custom_table::TableState;
use std::error;

use duckdb::{arrow::array::RecordBatch, Connection, Result as DuckResult};
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
    pub table_state: TableState,
    pub results: Vec<RecordBatch>,
    pub error: Option<String>,
    pub show_error_popup: bool,
    pub vertical_scroll: usize,
    pub horizontal_scroll: usize,
    pub vertical_scroll_state: ScrollbarState,
    pub horizontal_scroll_state: ScrollbarState,
    pub textarea: TextArea<'static>,
    pub file_explorer: FileExplorer,
}

impl App {
    pub fn new() -> AppResult<Self> {
        Ok(Self {
            running: true,
            connection: Connection::open_in_memory()?,
            input: String::new(),
            results: Vec::new(),
            error: None,
            show_error_popup: false,
            vertical_scroll: 0,
            horizontal_scroll: 0,
            vertical_scroll_state: ScrollbarState::default(),
            horizontal_scroll_state: ScrollbarState::default(),
            textarea: TextArea::default(),
            file_explorer: FileExplorer::new()?,
            table_state: TableState::default(),
        })
    }
    pub fn execute_query(&mut self) -> DuckResult<()> {
        let mut stmt = self.connection.prepare(&self.input)?;
        self.results = stmt.query_arrow([])?.collect();
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
}
