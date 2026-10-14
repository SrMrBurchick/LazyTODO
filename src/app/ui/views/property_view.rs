use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::{
    layout::{Constraint, Layout},
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::app::events::Request;
use crate::app::ui::styles;
use crate::app::ui::views::base::view::View;
use crate::app::ui::components::base::component::Component;
use crate::app::ui::components::user_input::{ UserInputComponent, EInputMode};

#[derive(Debug, Clone, Default)]
pub struct PropertyData {
    pub label: String,
    pub value : String,
    pub read_only: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PropertyView {
    data: PropertyData,
    user_input: Option<UserInputComponent>,
    selected: bool
}

impl PropertyView {
    pub fn new(data: &PropertyData) -> Self {
        PropertyView {
            data: data.clone(),
            user_input: None,
            selected: false
        }
    }
}

impl View for PropertyView {
    type ViewData = PropertyData;
    fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<crate::app::events::Request, String> {
        match &mut self.user_input {
            Some(component) => {
                if self.selected {
                    component.handle_key(key)
                } else {
                    Ok(Request::Nothing)
                }
            },
            None => {
                Ok(Request::Nothing)
            },
        }
    }

    fn render_data(&mut self, area: Rect, buf: &mut Buffer, data: &Self::ViewData) {
        let [label_area, value_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(3),
        ])
            .areas(area);

        Paragraph::new(Line::styled(
            format!("  {}", data.label.to_uppercase()),
            Style::default()
                .fg(styles::PROPERTY_LABEL_COLOR)
                .add_modifier(Modifier::BOLD),
        ))
            .render(label_area, buf);

        let editing = self.selected
        && self
            .user_input
            .as_ref()
            .is_some_and(|input| matches!(input.get_mode(), EInputMode::Editing));

        let border_color = if editing {
            styles::PROPERTY_EDITING_BORDER_COLOR
        } else if self.selected {
            styles::PROPERTY_SELECTED_BORDER_COLOR
        } else {
            styles::PROPERTY_BORDER_COLOR
        };

        let title = if data.read_only {
            " READ ONLY "
        } else if editing {
            " EDITING "
        } else if self.selected {
            " SELECTED "
        } else {
            ""
        };

        let title_color = if editing {
            styles::PROPERTY_EDITING_BORDER_COLOR
        } else if data.read_only {
            styles::PROPERTY_READ_ONLY_COLOR
        } else {
            border_color
        };

        let input_block = Block::bordered()
            .title(Line::styled(
                title,
                Style::default()
                    .fg(title_color)
                    .add_modifier(Modifier::BOLD),
            ))
            .border_style(Style::default().fg(border_color))
            .style(Style::default().bg(styles::PROPERTY_BG));

        let value = match &self.user_input {
            Some(input) if editing => input.get_data(),
            _ => data.value.clone(),
        };

        let value_color = if data.read_only {
            styles::PROPERTY_READ_ONLY_COLOR
        } else {
            styles::PROPERTY_VALUE_COLOR
        };

        Paragraph::new(Line::styled(
            value,
            Style::default().fg(value_color),
        ))
            .block(input_block)
            .render(value_area, buf);
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let data = self.data.clone();
        self.render_data(area, buf, &data);
    }


    fn render_frame(&mut self, frame: &mut Frame, area: Rect) {
        self.render(area, frame.buffer_mut());
    }

    fn mark_selected(&mut self, is_selected: bool) {
        if !self.data.read_only {
            match self.user_input {
                Some(_) => {},
                None => {
                    self.user_input = Some(UserInputComponent::new());
                },
            }
        }

        self.selected = is_selected;

    }
}
