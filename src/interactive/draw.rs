use super::list;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use super::{InteractiveMode, Mode};

impl Widget for &mut InteractiveMode {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.visible_options = area.height as usize;
        match &self.mode {
            Mode::List => list::render(self.clone(), area, buf),
            Mode::EditKey => list::render(self.clone(), area, buf),
            Mode::EditValue => list::render(self.clone(), area, buf),
            Mode::CreateNew => list::render(self.clone(), area, buf),
        }
    }
}