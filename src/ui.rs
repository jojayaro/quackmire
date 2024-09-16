use crate::table;

use ratatui::{
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Text,
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame,
};
use ratatui_explorer::Theme;

use crate::app::App;

const BG: Color = Color::Rgb(40, 40, 40); // Dark background
const FG: Color = Color::Rgb(235, 219, 178); // Light foreground
const RED: Color = Color::Rgb(204, 36, 29); // Gruvbox red
const GREEN: Color = Color::Rgb(152, 151, 26); // Gruvbox green
const YELLOW: Color = Color::Rgb(215, 153, 33); // Gruvbox yellow
const BLUE: Color = Color::Rgb(69, 133, 136); // Gruvbox blue
const PURPLE: Color = Color::Rgb(177, 98, 134); // Gruvbox purple
const AQUA: Color = Color::Rgb(104, 157, 106); // Gruvbox aqua
const GRAY: Color = Color::Rgb(146, 131, 116); // Gruvbox gray

fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    app.textarea.set_block(
        Block::bordered()
            .title("Query")
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(AQUA).bg(BG)),
    );
    app.textarea.set_line_number_style(Style::default().fg(FG));
    app.textarea.set_style(Style::default().fg(FG));

    let theme = Theme::default()
        .add_default_title()
        .with_block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(AQUA).bg(BG)),
        )
        .with_item_style(Style::default().fg(FG))
        .with_dir_style(Style::default().fg(AQUA));
    app.file_explorer.set_theme(theme);

    let layout = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(3),
    ]);
    let [top, middle, bottom] = layout.areas(frame.area());
    let horizontal = Layout::horizontal([Constraint::Percentage(25), Constraint::Percentage(75)]);
    let vertical = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]);
    let [explorer, right] = horizontal.areas(middle);
    let [query, results] = vertical.areas(right);

    let header = Paragraph::new("Quackmire")
        .block(
            Block::bordered()
                .title_alignment(Alignment::Left)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(AQUA).bg(BG)),
        )
        .style(Style::new().fg(FG).bg(BG));

    frame.render_widget(header, top);

    if !app.results.is_empty() {
        let batch = &app.results[0];
        let table = table::create_table(batch)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(AQUA).bg(BG))
                    .title("Results")
                    .title_alignment(Alignment::Left),
            )
            .style(Style::default().fg(FG).bg(BG))
            .header_style(Style::default().fg(PURPLE).bold());
        // .selected_style(Style::default().bg(FG));

        frame.render_stateful_widget(table, results, &mut app.table_state);
    } else {
        let no_results = Paragraph::new("")
            .block(
                Block::bordered()
                    .title("Results")
                    .title_alignment(Alignment::Left)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(AQUA).bg(BG));
        frame.render_widget(no_results, results);
    }
    frame.render_widget_ref(app.file_explorer.widget(), explorer);

    frame.render_widget(app.textarea.widget(), query);

    let footer = Paragraph::new("Super + Arrows to navigate | Fn + 2 for query | ^o to open file | ^s to enter file path into query | Esc to exit")
        .block(
            Block::bordered()
                .title_alignment(Alignment::Left)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(AQUA).bg(BG)),
        )
        .style(Style::default().fg(FG).bg(BG));
    frame.render_widget(footer, bottom);

    if app.show_error_popup {
        let area = frame.area(); // Changed from f.size() to f.area()
        let popup_area = popup_area(area, 60, 20);
        frame.render_widget(Clear, popup_area);
        let error_text = app.error.as_deref().unwrap_or("Unknown error");
        let error_popup = Paragraph::new(Text::raw(error_text))
            .block(Block::default().borders(Borders::ALL).title("Error"))
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::Red));
        frame.render_widget(error_popup, popup_area);
    }
}
