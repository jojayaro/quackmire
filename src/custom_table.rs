use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{Block, StatefulWidget, Widget},
};
use unicode_width::UnicodeWidthStr;

#[derive(Debug)]
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

pub struct Table<'a> {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    widths: Vec<u16>,
    block: Option<Block<'a>>,
    style: Style,
    header_style: Style,
}

impl<'a> Table<'a> {
    pub fn new(headers: Vec<String>, rows: Vec<Vec<String>>) -> Self {
        let widths = headers
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
            .collect();

        Self {
            headers,
            rows,
            widths,
            block: None,
            style: Style::default(),
            header_style: Style::default(),
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
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

impl StatefulWidget for Table<'_> {
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

        // Fill the entire table area with the background color
        buf.set_style(table_area, self.style);

        let mut y = table_area.top();
        let mut x = table_area.left();

        // Render headers
        for (i, header) in self
            .headers
            .iter()
            .enumerate()
            .skip(state.offset_x as usize)
        {
            let width = self.widths[i];
            if x + width > table_area.right() {
                break;
            }
            buf.set_string(x, y, header, self.header_style);
            x += width + 1;
        }

        // Render rows
        for row in self.rows.iter().skip(state.offset_y as usize) {
            if y >= table_area.bottom() - 1 {
                break;
            }
            y += 1;
            x = table_area.left();

            for (i, cell) in row.iter().enumerate().skip(state.offset_x as usize) {
                let width = self.widths[i];
                if x + width > table_area.right() {
                    break;
                }
                buf.set_string(x, y, cell, self.style);
                x += width + 1;
            }
        }
    }
}
