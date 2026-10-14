use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};
use crate::app::events::{Request, Response};

pub trait Component {
    fn new() -> Self;
    fn handle_key(&mut self, key: KeyEvent) -> Result<Request, String>;
    fn handle_response(&mut self, response: Response);
    fn render_frame(&mut self, _frame: &mut Frame, _area: Rect) {
        // STUB
    }
}
