use ratatui::style::{Color, Style};

use crate::color::ColorMap;

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

    /// Draw a multi-line string onto the layer at (x, y) with a uniform style.
    /// Spaces are transparent. Trailing newlines stripped; leading preserved for alignment.
    pub fn draw_ascii(&mut self, x: i32, y: i32, art: &str, style: Style) {
        self.draw_ascii_styled(x, y, art, style, None);
    }

    /// Draw a multi-line string with per-character coloring via an optional ColorMap.
    /// Characters not in the color map use the base_style.
    pub fn draw_ascii_styled(
        &mut self,
        x: i32,
        y: i32,
        art: &str,
        base_style: Style,
        colors: Option<&ColorMap>,
    ) {
        let art = art.strip_suffix('\n').unwrap_or(art);
        for (row, line) in art.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch == ' ' {
                    continue;
                }
                let px = x + col as i32;
                let py = y + row as i32;
                if px >= 0 && py >= 0 {
                    let style = match colors {
                        Some(cm) => match cm.get_color(ch, row, col) {
                            Some(Color::Rgb(r, g, b)) => base_style.fg(Color::Rgb(r, g, b)),
                            _ => base_style,
                        },
                        None => base_style,
                    };
                    self.set(px as u16, py as u16, ch, style);
                }
            }
        }
    }

    /// Composite this layer onto a ratatui Buffer. None cells are skipped (transparent).
    pub fn composite(&self, buf: &mut ratatui::buffer::Buffer, area: ratatui::layout::Rect) {
        self.composite_offset(buf, area, 0, 0);
    }

    /// Composite with a pixel offset (for parallax scrolling).
    pub fn composite_offset(
        &self,
        buf: &mut ratatui::buffer::Buffer,
        area: ratatui::layout::Rect,
        offset_x: i32,
        offset_y: i32,
    ) {
        for y in 0..self.height.min(area.height) {
            for x in 0..self.width.min(area.width) {
                if let Some(cell) = self.get(x, y) {
                    let bx = area.x as i32 + x as i32 + offset_x;
                    let by = area.y as i32 + y as i32 + offset_y;
                    if bx >= area.x as i32
                        && bx < (area.x + area.width) as i32
                        && by >= area.y as i32
                        && by < (area.y + area.height) as i32
                    {
                        if let Some(buf_cell) = buf.cell_mut((bx as u16, by as u16)) {
                            buf_cell.set_char(cell.ch);
                            buf_cell.set_style(cell.style);
                        }
                    }
                }
            }
        }
    }
}
