use crossterm::event::KeyCode;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::palette::tailwind::{BLUE, GREEN, SLATE};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{
    Block, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph,
    StatefulWidget, Widget, Wrap,
};
use ratatui::{DefaultTerminal, symbols};

use crate::app::events::{Request, Response};
use crate::app::ui::styles;
use crate::app::ui::views::base::view::{self, View};


pub trait ListViewItem {
    fn display(&self) -> String;
    fn render(&self) -> ListItem<'_>;
    fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<Request, String>;
    fn mark_selected(&self) -> Result<Request, String>;
}

pub struct ListView {
    state: ListState,
    data: Vec<Box<dyn ListViewItem>>,
    title: String,
}

const fn alternate_colors(i: usize) -> Color {
    if i.is_multiple_of(2) {
        styles::NORMAL_ROW_BG
    } else {
        styles::ALT_ROW_BG_COLOR
    }
}

impl ListView {
    pub fn new(title: &str) -> Self {
        ListView {
            state: ListState::default(),
            data: vec![],
            title: title.to_string()
        }
    }

    pub fn clean(&mut self) {
        self.data.clear();
    }

    pub fn add_item(&mut self, item: Box<dyn ListViewItem>) {
        self.data.push(item);
    }

    pub fn set_items(&mut self, items: Vec<Box<dyn ListViewItem>>) {
        self.data = items;
    }

    pub fn append_items(&mut self, items: Vec<Box<dyn ListViewItem>>) {
        self.data.extend(items);
    }

    pub fn select_none(&mut self) {
        self.state.select(None);
    }

    pub fn select_next(&mut self) {
        self.state.select_next();
    }

    pub fn select_previous(&mut self) {
        self.state.select_previous();
    }

    pub const fn select_first(&mut self) {
        self.state.select_first();
    }

    pub const fn select_last(&mut self) {
        self.state.select_last();
    }

    pub const fn selected(&self) -> Option<usize> {
        self.state.selected()
    }

    pub fn select(&mut self, id: Option<usize>) {
        match self.state.select(id) {
            _ => {},
        };
    }

    fn handle_navigation(&mut self, key: crossterm::event::KeyEvent)  -> Result<crate::app::events::Request, String> {
        let prev_selected = self.state.selected();

        match key.code {
            KeyCode::Char('h') => self.select_none(),
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
            KeyCode::Char('g') | KeyCode::Home => self.select_first(),
            KeyCode::Char('G') | KeyCode::End => self.select_last(),
            _ => {
            }
        }

        match prev_selected {
            Some(selected) => {
                match self.state.selected() {
                    Some(current_selected) => {
                        if selected != current_selected {
                            match self.data.get(current_selected) {
                                Some(item) => {
                                    return item.mark_selected();
                                },
                                None => {},
                            }
                        }
                    },
                    None => {},
                }
            },
            None => {},
        }

        Ok(Request::Nothing)
    }

}

impl View for ListView {
    type ViewData = Vec<Box<dyn ListViewItem>>;
    fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<crate::app::events::Request, String> {
        match self.handle_navigation(key) {

            Ok(request) => {
                if request != Request::Nothing {
                    return Ok(request);
                }
            },
            Err(_) => {},
        }

        match self.state.selected() {
            Some(id) => {
                match self.data.get_mut(id) {
                    Some(item) => {
                        return item.handle_key(key);
                    },
                    None => {},
                }
            },
            None => {},
        }

        Ok(Request::Nothing)
    }

    fn render_data(&mut self, area: Rect, buf: &mut Buffer, data: &Self::ViewData) {
        let block = Block::new()
            .title(Line::raw(self.title.as_str()).centered())
            .borders(Borders::ALL)
            .border_set(symbols::border::EMPTY)
            .border_style(styles::TODO_HEADER_STYLE)
            .bg(styles::NORMAL_ROW_BG);

        // Iterate through all elements in the `items` and stylize them.
        let items: Vec<ListItem> = data 
            .iter()
            .enumerate()
            .map(|(i, item)| {
                item.render().bg(alternate_colors(i))
            })
            .collect();

        // // Create a List from all list items and highlight the currently selected one
        let list = List::new(items)
            .block(block)
            .highlight_style(styles::SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);


        // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
        // same method name `render`.
        StatefulWidget::render(list, area, buf, &mut self.state);

    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw(self.title.as_str()).centered())
            .borders(Borders::ALL)
            .border_set(symbols::border::EMPTY)
            .border_style(styles::TODO_HEADER_STYLE)
            .bg(styles::NORMAL_ROW_BG);

        // Iterate through all elements in the `items` and stylize them.
        let items: Vec<ListItem> = self
            .data
            .iter()
            .enumerate()
            .map(|(i, item)| {
                item.render().bg(alternate_colors(i))
            })
            .collect();

        // // Create a List from all list items and highlight the currently selected one
        let list = List::new(items)
            .block(block)
            .highlight_style(styles::SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
        // same method name `render`.
        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}
