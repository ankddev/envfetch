mod draw;
mod list;
mod tests;

use std::io;

use super::variables::{get_variables, set_variable, delete_variable};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{Frame, Terminal};
use crate::models::ErrorKind;

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
    input_buffer: String,
    error_message: Option<String>,
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
            input_buffer: String::new(),
            error_message: None,
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
        // Clear previous error message
        self.error_message = None;

        match self.mode {
            Mode::List => self.handle_list_mode_keys(key_event),
            Mode::EditKey => self.handle_edit_key_mode_keys(key_event),
            Mode::EditValue => self.handle_edit_value_mode_keys(key_event),
            Mode::CreateNew => self.handle_create_mode_keys(key_event),
        }
    }

    fn handle_list_mode_keys(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Char('Q') 
                if key_event.modifiers == KeyModifiers::CONTROL => {
                self.exit()
            }
            KeyCode::Down => self.down(),
            KeyCode::Up => self.up(),
            KeyCode::Left => self.scroll_value_left(),
            KeyCode::Right => self.scroll_value_right(),
            KeyCode::Char('r') | KeyCode::Char('R')
                if key_event.modifiers == KeyModifiers::CONTROL => {
                self.reload()
            }
            KeyCode::Char('e') => {
                // Enter edit key mode
                if !self.entries.is_empty() {
                    self.mode = Mode::EditKey;
                    self.input_buffer = self.entries[self.current_index].0.clone();
                }
            }
            KeyCode::Char('v') => {
                // Enter edit value mode
                if !self.entries.is_empty() {
                    self.mode = Mode::EditValue;
                    self.input_buffer = self.entries[self.current_index].1.clone();
                }
            }
            KeyCode::Char('n') => {
                // Enter create new variable mode
                self.mode = Mode::CreateNew;
                self.input_buffer = String::new();
            }
            KeyCode::Char('d') => {
                // Delete current variable
                if !self.entries.is_empty() {
                    let old_key = &self.entries[self.current_index].0;
                    match delete_variable(old_key.to_string(), false) {
                        Ok(_) => {
                            self.reload();
                        }
                        Err(e) => {
                            self.error_message = Some(format!("Delete failed: {:?}", e));
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn handle_edit_key_mode_keys(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Enter => {
                // Save new key
                if !self.entries.is_empty() {
                    let old_key = &self.entries[self.current_index].0;
                    let old_value = &self.entries[self.current_index].1;
                    let new_key = self.input_buffer.trim().to_string();
                    
                    if !new_key.is_empty() {
                        match set_variable(&new_key, old_value, false, None) {
                            Ok(_) => {
                                // Delete the old variable
                                let _ = delete_variable(old_key.to_string(), false);
                                self.mode = Mode::List;
                                self.reload();
                            }
                            Err(e) => {
                                self.error_message = Some(format!("Set variable failed: {:?}", e));
                            }
                        }
                    }
                }
            }
            KeyCode::Esc => {
                // Cancel edit
                self.mode = Mode::List;
            }
            KeyCode::Char(c) => {
                self.input_buffer.push(c);
            }
            KeyCode::Backspace => {
                self.input_buffer.pop();
            }
            _ => {}
        }
    }

    fn handle_edit_value_mode_keys(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Enter => {
                // Save new value
                if !self.entries.is_empty() {
                    let key = &self.entries[self.current_index].0;
                    let new_value = self.input_buffer.trim().to_string();
                    
                    match set_variable(key, &new_value, false, None) {
                        Ok(_) => {
                            self.mode = Mode::List;
                            self.reload();
                        }
                        Err(e) => {
                            self.error_message = Some(format!("Set variable failed: {:?}", e));
                        }
                    }
                }
            }
            KeyCode::Esc => {
                // Cancel edit
                self.mode = Mode::List;
            }
            KeyCode::Char(c) => {
                self.input_buffer.push(c);
            }
            KeyCode::Backspace => {
                self.input_buffer.pop();
            }
            _ => {}
        }
    }

    fn handle_create_mode_keys(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Enter => {
                // Add new variable
                let parts: Vec<&str> = self.input_buffer.splitn(2, '=').collect();
                if parts.len() == 2 {
                    let key = parts[0].trim();
                    let value = parts[1].trim();
                    
                    if !key.is_empty() && !value.is_empty() {
                        match set_variable(key, value, false, None) {
                            Ok(_) => {
                                self.mode = Mode::List;
                                self.reload();
                            }
                            Err(e) => {
                                self.error_message = Some(format!("Set variable failed: {:?}", e));
                            }
                        }
                    }
                }
            }
            KeyCode::Esc => {
                // Cancel create
                self.mode = Mode::List;
            }
            KeyCode::Char(c) => {
                self.input_buffer.push(c);
            }
            KeyCode::Backspace => {
                self.input_buffer.pop();
            }
            _ => {}
        }
    }

    // Existing methods (down, up, reload, etc.) remain the same as in previous implementation
    // ... 
}