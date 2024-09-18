use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{Block, StatefulWidget, Widget},
};
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone)]
pub struct TableState {
    pub offset_x: u16,
    pub offset_y: u16,
}

impl Default for TableState {
    fn default() -> Self {
        Self {
            offset_x: 0,
            offset_y: 0,
        }
    }
}

pub struct Table {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub widths: Vec<u16>,
    pub block: Option<Block<'static>>,
    pub style: Style,
    pub header_style: Style,
}

impl Table {
    pub fn new(headers: Vec<String>, rows: Vec<Vec<String>>) -> Self {
        let widths = if headers.is_empty() {
            vec![]
        } else {
            headers
                .iter()
                .enumerate()
                .map(|(i, h)| {
                    std::cmp::max(
                        h.width() as u16,
                        rows.iter()
                            .map(|r| r.get(i).map(|c| c.width() as u16).unwrap_or(0))
                            .max()
                            .unwrap_or(0),
                    )
                })
                .collect()
        };

        Self {
            headers,
            rows,
            widths,
            block: None,
            style: Style::default(),
            header_style: Style::default(),
        }
    }

    pub fn block(mut self, block: Block<'static>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn header_style(mut self, style: Style) -> Self {
        self.header_style = style;
        self
    }
}

impl StatefulWidget for Table {
    type State = TableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let table_area = match self.block {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        if table_area.width < 1 || table_area.height < 1 {
            return;
        }

        buf.set_style(table_area, self.style);

        let mut y = table_area.top();
        let visible_width = table_area.width;
        let visible_height = table_area.height;

        // Render headers
        let mut x = table_area.left().saturating_sub(state.offset_x);
        for (i, header) in self.headers.iter().enumerate() {
            let width = self.widths[i];
            if x >= table_area.left() && x + width <= table_area.right() {
                buf.set_string(x, y, header, self.header_style);
            }
            x += width + 1;
        }

        // Render rows
        if !self.rows.is_empty() {
            for row in self.rows.iter().skip(state.offset_y as usize) {
                y += 1;
                if y >= table_area.bottom() {
                    break;
                }
                let mut x = table_area.left().saturating_sub(state.offset_x);
                for (i, cell) in row.iter().enumerate() {
                    if i < self.widths.len() {
                        let width = self.widths[i];
                        if x >= table_area.left() && x + width <= table_area.right() {
                            buf.set_string(x, y, cell, self.style);
                        }
                        x += width + 1;
                    }
                }
            }
        }
    }
}
