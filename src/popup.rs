use ratatui::layout::{Constraint, Direction, Flex, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

#[derive(Debug)]
pub struct FileNamePopup {
    pub input: String,
}

impl FileNamePopup {
    pub fn new() -> Self {
        Self {
            input: String::new(),
        }
    }

    pub fn render(&self, frame: &mut ratatui::Frame) {
        let area = frame.area();
        let popup_area = popup_area(area, 60, 20);
        frame.render_widget(Clear, popup_area);
        frame.render_widget(
            Block::default()
                .title("Save File")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::DarkGray)),
            popup_area,
        );

        let inner_area = Rect::new(
            popup_area.x + 2,
            popup_area.y + 2,
            popup_area.width - 4,
            popup_area.height - 4,
        );
        frame.render_widget(
            Paragraph::new(Text::raw(&format!("File name: {}", self.input)))
                .style(Style::default().fg(Color::White)),
            inner_area,
        );
    }
}
