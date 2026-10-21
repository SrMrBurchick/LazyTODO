use crossterm::event::KeyEvent;
use ratatui::{Frame, buffer::Buffer, layout::Rect};

pub trait View {
    type ViewData;
    fn render(&mut self, area: Rect, buf: &mut Buffer);
    fn render_data(&mut self, area: Rect, buf: &mut Buffer, data: &Self::ViewData);
    fn render_frame(&mut self, _frame: &mut Frame, _area: Rect) {
        // STUB
    }
}

pub trait ViewState {
    fn mark_selected(&mut self, is_selected: bool);
    fn handle_key(&mut self, key: KeyEvent) -> Result<crate::app::events::Request, String> ;
	fn is_seleceted(&self) -> bool { false }
	fn on_callback(&mut self) {}
}
