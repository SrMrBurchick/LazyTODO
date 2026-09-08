use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect};

pub trait View {
    fn handle_key(&mut self, key: KeyEvent) -> Result<crate::app::events::Request, String> ;
    fn render(&mut self, area: Rect, buf: &mut Buffer);
}

