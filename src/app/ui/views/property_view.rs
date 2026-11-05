use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::{
    layout::{Constraint, Layout},
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};
use tracing::info;

use crate::app::events::Request;
use crate::app::ui::styles::{self, get_ui_width, wrap_lines};
use crate::app::ui::views::base::view::{View, ViewStateController};
use crate::app::ui::components::base::component::Component;
use crate::app::ui::components::user_input::{ UserInputComponent, EInputMode};

use super::scroll_view::ScrollView;

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
	scroll_view: ScrollView,
    selected: bool
}

impl PropertyView {
    pub fn new(data: PropertyData) -> Self {
        PropertyView {
            data: data.clone(),
            user_input: None,
			scroll_view: ScrollView::new(),
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

	pub fn has_changed_data(&self) -> bool {
		match &self.user_input {
			Some(component) => {
				component.is_data_updated()
			}
			_=> {
				false
			}
		}
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
            Constraint::Fill(1),
        ]).areas(area);

		let data_updated: bool;
		let mut label: String = self.data.label.trim_end_matches(':').to_uppercase();

		match &self.user_input {
			Some(component) => {
				data_updated = component.is_data_updated();
			}
			_=> {
				data_updated = false;
			}
		}

        Paragraph::new(Line::styled(
            label,
            Style::default()
                .fg(styles::PROPERTY_LABEL_COLOR)
                .add_modifier(Modifier::BOLD),
        )).render(label_area, frame.buffer_mut());

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
        } else {
			if data_updated {
				"UPDATED*"
			} else {
				""
			}
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

		let input_inner = input_block.inner(value_area);
		let max_width = input_inner.width as usize;

		if let Some(component) = &mut self.user_input {
			component.set_max_width(max_width);
		}

		let value_list = wrap_lines(
			value.as_str(),
			max_width,
		);

		let lines: Vec<Line> = value_list
			.iter()
			.map(|line| {
				Line::styled(
					line.clone(),
					Style::default().fg(value_color),
				)
			})
			.collect();

		Paragraph::new(lines)
			.block(input_block)
			.scroll((self.scroll_view.get_scroll() as u16, 0))
			.render(value_area, frame.buffer_mut());

		self.scroll_view.update(
			value_list.len(),
			input_inner.height as usize,
		);
		// self.scroll_view.render_frame(frame, input_inner);

		if editing {
			if let Some(input) = &self.user_input {
				let cursor = input.get_cursor_position();

				let text_before_cursor: String = input
					.get_data()
					.chars()
					.take(cursor)
					.collect();

				let cursor_lines = wrap_lines(
					text_before_cursor.as_str(),
					max_width,
				);

				let cursor_y = cursor_lines
					.len()
					.saturating_sub(1);

				self.scroll_view.track(cursor_y);

				let cursor_x = cursor_lines
					.last()
					.map(|line| get_ui_width(line.as_str()))
					.unwrap_or(0);

				let visible_cursor_y =
					self.scroll_view.get_visible_position(cursor_y);

				frame.set_cursor_position(Position::new(
					input_inner.x + cursor_x as u16,
					input_inner.y + visible_cursor_y as u16,
				));

				// frame.set_cursor_position(Position::new(
				// 	input_inner.x + cursor_x as u16,
				// 	input_inner.y + cursor_y as u16,
				// ))
			}
		}
    }
}

impl ViewStateController for PropertyView {
    fn mark_selected(&mut self, is_selected: bool) {
        if !self.data.read_only {
            match self.user_input {
                Some(_) => {},
                None => {
                    self.user_input = Some(UserInputComponent::new());
					match &mut self.user_input {
						Some(component) => {
							component.set_data(self.data.value.clone());
						}
						_=> {}
					}
                },
            }
        }

        self.selected = is_selected;
    }

	fn is_seleceted(&self) -> bool {
		self.selected
	}

	fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<crate::app::events::Request, String> {
		if self.get_mode() == Some(EInputMode::Editing) {
			self.scroll_view.handle_key(key);
		}

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
