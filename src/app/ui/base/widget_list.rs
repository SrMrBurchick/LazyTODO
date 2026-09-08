use ratatui::widgets::{ListItem, ListState};

use crate::app::events::Request;

pub trait WidgetListItem {
    fn display(&self) -> String;
    fn render(&self) -> ListItem<'_>;
    fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<Request, String>;
}

pub struct WidgetList {
    pub items: Vec<Box<dyn WidgetListItem>>,
    pub state: ListState
}

impl WidgetList {
    pub fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<Request, String> {
        match self.state.selected() {
            Some(id) => {
                match self.items.get_mut(id) {
                    Some(item) => {
                        item.handle_key(key)
                    },
                    None => {
                        Ok(Request::Nothing)
                    },
                }
            },
            None => {
                Ok(Request::Nothing)
            },
        }
    }
}

impl Default for WidgetList {
    fn default() -> Self {
        WidgetList { items: vec![], state: ListState::default() }
    }
}

