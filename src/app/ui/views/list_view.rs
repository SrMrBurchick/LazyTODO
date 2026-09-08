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



use crate::app::events::Response;
use crate::app::ui::base::widget_list::*;
use crate::app::ui::styles;
use crate::app::ui::views::base::view::{self, View};

pub struct ListView {
    list: WidgetList,
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
            list: WidgetList::default(),
            title: title.to_string()
        }
    }

    pub fn clean(&mut self) {
        self.list.items.clear();
    }

    pub fn add_item(&mut self, item: Box<dyn WidgetListItem>) {
        self.list.items.push(item);
    }

    pub fn set_items(&mut self, items: Vec<Box<dyn WidgetListItem>>) {
        self.list.items = items;
    }

    pub fn append_items(&mut self, items: Vec<Box<dyn WidgetListItem>>) {
        self.list.items.extend(items);
    }

    pub fn select_none(&mut self) {
        self.list.state.select(None);
    }

    pub fn select_next(&mut self) {
        self.list.state.select_next();
    }

    pub fn select_previous(&mut self) {
        self.list.state.select_previous();
    }

    pub const fn select_first(&mut self) {
        self.list.state.select_first();
    }

    pub const fn select_last(&mut self) {
        self.list.state.select_last();
    }

    pub const fn selected(&self) -> Option<usize> {
        self.list.state.selected()
    }

    fn handle_navigation(&mut self, key: crossterm::event::KeyEvent) {
        match key.code {
            KeyCode::Char('h') | KeyCode::Left => self.select_none(),
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
            KeyCode::Char('g') | KeyCode::Home => self.select_first(),
            KeyCode::Char('G') | KeyCode::End => self.select_last(),
            _ => {
            }
        }
    }

}

impl View for ListView {
    fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<crate::app::events::Request, String> {
        self.handle_navigation(key);
        self.list.handle_key(key)
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw(self.title.as_str()).centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(styles::TODO_HEADER_STYLE)
            .bg(styles::NORMAL_ROW_BG);

        // Iterate through all elements in the `items` and stylize them.
        let items: Vec<ListItem> = self
            .list
            .items
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
        StatefulWidget::render(list, area, buf, &mut self.list.state);
    }
}
