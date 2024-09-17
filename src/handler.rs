use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui_explorer::Input;
use std::fs::read_to_string;

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match (key_event.code, key_event.modifiers) {
        (KeyCode::Esc, _) => {
            if app.show_error_popup {
                app.toggle_error_popup();
            } else {
                app.quit();
            }
        }
        (KeyCode::F(2), _) => {
            if let Err(e) = app.run_query() {
                app.results.clear();
                app.error = Some(format!("Error: {}", e));
                app.toggle_error_popup();
            }
        }
        (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
            if let Some(path) = app.file_explorer.current().path().to_str() {
                app.textarea.insert_str(path);
            }
        }
        (KeyCode::Char('o'), KeyModifiers::CONTROL) => {
            if app.file_explorer.current().path().is_file() {
                app.textarea.select_all();
                app.textarea.cut();
                if let Ok(content) = read_to_string(app.file_explorer.current().path()) {
                    app.textarea.insert_str(&content);
                }
            }
        }
        (KeyCode::Down, KeyModifiers::SUPER) => app.file_explorer.handle(Input::Down)?,
        (KeyCode::Up, KeyModifiers::SUPER) => app.file_explorer.handle(Input::Up)?,
        (KeyCode::Left, KeyModifiers::SUPER) => app.file_explorer.handle(Input::Left)?,
        (KeyCode::Right, KeyModifiers::SUPER) => app.file_explorer.handle(Input::Right)?,
        _ => {}
    }

    app.textarea.input(key_event);
    app.input = app.textarea.lines().join("\n");

    Ok(())
}
