mod draw;
mod list;
mod tests;

use std::io;

use super::variables::get_variables;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{Frame, Terminal};

#[derive(Clone)]
pub enum Mode {
    List,
    EditKey,
    EditValue,
    CreateNew,
}

impl Default for Mode {
    fn default() -> Self {
        Self::List
    }
}

#[derive(Clone)]
pub struct InteractiveMode {
    mode: Mode,
    exit: bool,
    entries: Vec<(String, String)>,
    current_index: usize,
    scroll_offset: usize,
    visible_options: usize,
    truncation_len: usize,
    value_scroll_offset: usize,
}

impl Default for InteractiveMode {
    fn default() -> Self {
        InteractiveMode {
            mode: Mode::List,
            exit: false,
            entries: get_variables(),
            current_index: 0,
            scroll_offset: 0,
            visible_options: 30,
            truncation_len: 30,
            value_scroll_offset: 0,
        }
    }
}

impl InteractiveMode {
    pub fn init() -> Self {
        Self::default()
    }

    pub fn run<B>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()>
    where
        B: ratatui::backend::Backend,
    {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Char('Q') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.exit = true;
            }
            KeyCode::Down => {
                self.current_index = (self.current_index + 1).min(self.entries.len().saturating_sub(1));
                self.value_scroll_offset = 0;
                
                // Adjust scroll offset if needed
                let visible_area = self.visible_options.saturating_sub(8);
                let scroll_trigger = self.scroll_offset + (visible_area.saturating_sub(4));
                
                if self.current_index > scroll_trigger {
                    self.scroll_offset += 1;
                }
            }
            KeyCode::Up => {
                if self.current_index > 0 {
                    self.current_index -= 1;
                    self.value_scroll_offset = 0;
                    
                    if self.current_index < self.scroll_offset {
                        self.scroll_offset = self.current_index;
                    }
                }
            }
            KeyCode::Left => {
                if self.value_scroll_offset > 0 {
                    self.value_scroll_offset -= 1;
                }
            }
            KeyCode::Right => {
                if let Some((_, value)) = self.entries.get(self.current_index) {
                    if self.value_scroll_offset < value.len() {
                        self.value_scroll_offset += 1;
                    }
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.entries = super::variables::get_variables();
                self.current_index = 0;
                self.scroll_offset = 0;
                self.value_scroll_offset = 0;
            }
            _ => {}
        }
    }
}