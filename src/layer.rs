use ratatui::style::Style;

#[derive(Clone)]
pub struct StyledCell {
    pub ch: char,
    pub style: Style,
}

pub struct Layer {
    pub width: u16,
    pub height: u16,
    cells: Vec<Option<StyledCell>>,
}

impl Layer {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            cells: vec![None; (width as usize) * (height as usize)],
        }
    }

    pub fn clear(&mut self) {
        self.cells.fill(None);
    }

    pub fn set(&mut self, x: u16, y: u16, ch: char, style: Style) {
        if x < self.width && y < self.height {
            let idx = (y as usize) * (self.width as usize) + (x as usize);
            self.cells[idx] = Some(StyledCell { ch, style });
        }
    }

    pub fn get(&self, x: u16, y: u16) -> Option<&StyledCell> {
        if x < self.width && y < self.height {
            let idx = (y as usize) * (self.width as usize) + (x as usize);
            self.cells[idx].as_ref()
        } else {
            None
        }
    }

    /// Draw a multi-line string onto the layer at (x, y).
    /// Each line is a row. Spaces are treated as transparent (not written).
    /// Leading/trailing newlines are stripped so r#" string literals work cleanly.
    pub fn draw_ascii(&mut self, x: i32, y: i32, art: &str, style: Style) {
        let art = art.strip_prefix('\n').unwrap_or(art);
        let art = art.strip_suffix('\n').unwrap_or(art);
        for (row, line) in art.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch == ' ' {
                    continue;
                }
                let px = x + col as i32;
                let py = y + row as i32;
                if px >= 0 && py >= 0 {
                    self.set(px as u16, py as u16, ch, style);
                }
            }
        }
    }

    /// Composite this layer onto a ratatui Buffer. None cells are skipped (transparent).
    pub fn composite(&self, buf: &mut ratatui::buffer::Buffer, area: ratatui::layout::Rect) {
        for y in 0..self.height.min(area.height) {
            for x in 0..self.width.min(area.width) {
                if let Some(cell) = self.get(x, y) {
                    let buf_cell = buf.cell_mut((area.x + x, area.y + y));
                    if let Some(buf_cell) = buf_cell {
                        buf_cell.set_char(cell.ch);
                        buf_cell.set_style(cell.style);
                    }
                }
            }
        }
    }
}
