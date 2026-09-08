use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::app::events::{Request, Response};

pub trait Section {
    fn initialize(&mut self);
    fn get_name(&self) -> String;
    fn handle_key(&mut self, key: KeyEvent) -> Result<Request, String>;
    fn render(&mut self, area: Rect, buf: &mut Buffer);
    fn handle_response(&mut self, response: Response);
    fn is_focused(&self) -> bool;
    fn set_focus(&mut self, focus: bool);
    fn toggle_focus(&mut self);
    fn reset(&mut self);
}

