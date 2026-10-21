use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::{
    layout::{Constraint, Layout},
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::app::events::Request;
use crate::app::ui::styles;
use crate::app::ui::views::base::view::{View, ViewState};
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
    pub data: PropertyData,
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

    pub fn get_mode(&self) -> Option<EInputMode> {
        match &self.user_input {
            Some(input) => {
                return Some(input.get_mode())
            },
            None => {
            },
        }

        None
    }
}

impl View for PropertyView {
    type ViewData = PropertyData;

    fn render_data(&mut self, area: Rect, buf: &mut Buffer, data: &Self::ViewData) {
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
    }


    fn render_frame(&mut self, frame: &mut Frame, area: Rect) {
        let [label_area, value_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(3),
        ])
            .areas(area);

        Paragraph::new(Line::styled(
            format!("  {}", self.data.label.to_uppercase()),
            Style::default()
                .fg(styles::PROPERTY_LABEL_COLOR)
                .add_modifier(Modifier::BOLD),
        ))
            .render(label_area, frame.buffer_mut());

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

        let title = if self.data.read_only {
            " READ ONLY "
        } else if editing {
            " EDITING "
        } else if self.selected {
            ""
        } else {
            ""
        };

        let title_color = if editing {
            styles::PROPERTY_EDITING_BORDER_COLOR
        } else if self.data.read_only {
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
            _ => self.data.value.clone(),
        };

        let value_color = if self.data.read_only {
            styles::PROPERTY_READ_ONLY_COLOR
        } else {
            styles::PROPERTY_VALUE_COLOR
        };

        Paragraph::new(Line::styled(
            value,
            Style::default().fg(value_color),
        ))
            .block(input_block)
            .render(value_area, frame.buffer_mut());

        match self.get_mode() {
            Some(mode) => {
                match mode {
                    EInputMode::Normal => {}

                    EInputMode::Editing => {
                        match &self.user_input {
                            Some(input) => {
                                frame.set_cursor_position(Position::new(
                                    value_area.x + input.get_cursor_position() as u16 + 1,
                                    value_area.y + 1,
                                ))

                            },
                            None => {},
                        }
                    } ,

                    _ => {},
                }
            },
            None => {},
        }

    }
}

impl ViewState for PropertyView {
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

	fn is_seleceted(&self) -> bool {
		self.selected
	}

	fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<crate::app::events::Request, String> {
		match &mut self.user_input {
			Some(component) => {
				if self.selected {
					let response = component.handle_key(key);
					self.data.value = component.get_data();
					return response;
				} else {
					Ok(Request::Nothing)
				}
			},
			None => {
				Ok(Request::Nothing)
			},
		}
	}
}
