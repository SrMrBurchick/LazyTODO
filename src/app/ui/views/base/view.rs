use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect};

pub trait View {
    type ViewData;
    fn handle_key(&mut self, key: KeyEvent) -> Result<crate::app::events::Request, String> ;
    fn render(&mut self, area: Rect, buf: &mut Buffer);
    fn render_data(&mut self, area: Rect, buf: &mut Buffer, data: &Self::ViewData);
}

