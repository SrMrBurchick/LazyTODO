use crossterm::event::KeyEvent;
use ratatui::{Frame, buffer::Buffer, layout::Rect};

use crate::app::{events::{Request, Response}, ui::sections::ESectionId};


#[derive(Debug)]
pub struct SectionContext {
}

pub trait Section {
    fn initialize(&mut self);
    fn handle_key(&mut self, key: KeyEvent) -> Result<Request, String>;
    fn render(&mut self, area: Rect, frame: &mut Frame);
    fn handle_response(&mut self, response: &Response) -> Result<Request, String>;
    fn reset(&mut self);
    fn get_id(&self) -> Option<ESectionId>;
    fn mark_selected(&mut self, is_selected: bool);
    fn get_section_context(&self) -> Option<SectionContext> {
        None
    }
}

