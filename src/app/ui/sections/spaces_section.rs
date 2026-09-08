use crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{
    Block, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph,
    StatefulWidget, Widget, Wrap,
};
use ratatui::symbols;

use crate::app::base::Target;
use crate::app::events::{DatabaseRequest, Request};
use crate::app::ui::views::base::view::View;
use crate::app::{events::Response, ui::{base::widget_list::WidgetListItem, sections::base::section::Section, styles, views::list_view::ListView}};

enum ESpaces {
    Home = 0,
    Projects = 1,
    Standalone = 2,
    Scheduled = 3
}

impl TryFrom<usize> for ESpaces {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ESpaces::Home),
            1 => Ok(ESpaces::Projects),
            2 => Ok(ESpaces::Standalone),
            3 => Ok(ESpaces::Scheduled),
            _ => Err(format!("Invalid space: {}", value))
        }
    }
}

struct Space {
    pub title: String,
    pub icon: String,
    space_type: ESpaces
}

impl Space {
    pub fn new(title: &str, icon: &str, space_type: ESpaces) -> Self {
        Space {
            title: title.to_string(),
            icon: icon.to_string(),
            space_type: space_type
        }
    }
}

impl std::fmt::Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.icon, self.title)
    }
}

impl From<&Space> for ListItem<'_> {
    fn from(value: &Space) -> Self {
        ListItem::new(Line::styled(format!("{value}"), styles::TEXT_FG_COLOR))
    }
}

impl WidgetListItem for Space {
    fn display(&self) -> String {
        format!("{self}")
    }
    fn render(&self) -> ratatui::widgets::ListItem<'_> {
        ListItem::from(self)
    }

    fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<crate::app::events::Request, String> {
        match key.code {
            KeyCode::Enter => {
                match self.space_type {
                    ESpaces::Home => {
                        return Ok(Request::Database(DatabaseRequest::Get(Target::All, None)));
                    },
                    ESpaces::Projects => {
                        return Ok(Request::Database(DatabaseRequest::Get(Target::Project, None)));
                    },
                    ESpaces::Standalone => {
                        return Ok(Request::Database(DatabaseRequest::Get(Target::Task, None)));
                    }
                    _ => {
                        Ok(Request::Nothing)
                    },
                }
            }
            _ => {
                Ok(Request::Nothing)
            }
        }
    }
}


pub struct SpacesSection {
    view: ListView,
    focused: bool
}

impl SpacesSection {
    pub fn new() -> Self {
        SpacesSection {
            view: ListView::new("Spaces"),
            focused: true
        }
    }
}

impl Section for SpacesSection {
    fn initialize(&mut self) {
        self.view.add_item(Box::new(Space::new("HOME", "⌂", ESpaces::Home)));
        self.view.add_item(Box::new(Space::new("Projects", "◈", ESpaces::Projects)));
        self.view.add_item(Box::new(Space::new("STANDALONE", "□", ESpaces::Standalone)));
        self.view.add_item(Box::new(Space::new("SCHEDULED", "⚑", ESpaces::Scheduled)));
    }

    fn handle_response(&mut self, response: Response) {
    }

    fn is_focused(&self) -> bool {
        self.focused
    }

    fn reset(&mut self) {
    }

    fn set_focus(&mut self, focus: bool) {
        self.focused = focus;
    }

    fn toggle_focus(&mut self) {
        self.focused = !self.focused;
    }

    fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<Request, String> {
        self.view.handle_key(key)
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        // content
        self.view.render(area, buf);
    }

    fn get_name(&self) -> String {
        "Spaces".to_uppercase().to_string()
    }
}
