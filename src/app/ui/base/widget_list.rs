use ratatui::widgets::{ListItem, ListState};

pub trait WidgetListItem {
    fn display(&self) -> String;
    fn render(&self) -> ListItem<'_>;
}

pub struct WidgetList {
    pub items: Vec<Box<dyn WidgetListItem>>,
    pub state: ListState
}

impl Default for WidgetList {
    fn default() -> Self {
        WidgetList { items: vec![], state: ListState::default() }
    }
}

