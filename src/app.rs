use crate::custom_table::TableState;
use std::error;

use duckdb::arrow::error::ArrowError;
use duckdb::arrow::record_batch::RecordBatchIterator;
use std::error::Error;
use std::vec::IntoIter;

use duckdb::{arrow::array::RecordBatch, Connection};
use ratatui::widgets::ScrollbarState;
use ratatui_explorer::FileExplorer;
use tui_textarea::TextArea;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
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
    pub results_iterator: Option<RecordBatchIterator<IntoIter<Result<RecordBatch, ArrowError>>>>,
    pub current_batch: Option<RecordBatch>,
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
            results_iterator: None,
            current_batch: None,
        })
    }
    pub fn run_query(&mut self) -> Result<(), Box<dyn Error>> {
        let mut stmt = self.connection.prepare(&self.input)?;
        let results: Vec<RecordBatch> = stmt.query_arrow([])?.collect();
        if !results.is_empty() {
            let schema = results[0].schema();
            let result_vec: Vec<Result<RecordBatch, ArrowError>> =
                results.into_iter().map(Ok).collect();
            let result_iter: IntoIter<Result<RecordBatch, ArrowError>> = result_vec.into_iter();

            self.results_iterator = Some(RecordBatchIterator::new(result_iter, schema.clone()));
            self.current_batch = self
                .results_iterator
                .as_mut()
                .and_then(|iter| iter.next().transpose().ok())
                .flatten();
        } else {
            self.results_iterator = None;
            self.current_batch = None;
        }
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
        if amount > 0 {
            if self.vertical_scroll == self.current_batch.as_ref().map_or(0, |b| b.num_rows() - 1) {
                if let Some(iter) = &mut self.results_iterator {
                    if let Some(Ok(next_batch)) = iter.next() {
                        self.current_batch = Some(next_batch);
                        self.vertical_scroll = 0;
                    }
                }
            } else {
                self.vertical_scroll = self.vertical_scroll.saturating_add(amount as usize);
            }
        } else {
            if self.vertical_scroll == 0 {
            } else {
                self.vertical_scroll = self.vertical_scroll.saturating_sub((-amount) as usize);
            }
        }
        self.vertical_scroll_state = self.vertical_scroll_state.position(self.vertical_scroll);
    }

    pub fn scroll_horizontal(&mut self, amount: isize) {
        if amount > 0 {
            // Scrolling right
            self.horizontal_scroll = self.horizontal_scroll.saturating_add(amount as usize);
        } else {
            // Scrolling left
            self.horizontal_scroll = self.horizontal_scroll.saturating_sub((-amount) as usize);
        }
        self.horizontal_scroll_state = self.horizontal_scroll_state.position(self.horizontal_scroll);
    }
}
