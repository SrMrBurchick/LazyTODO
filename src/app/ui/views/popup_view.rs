use std::cell::RefCell;
use std::rc::Rc;

use crossterm::event::KeyCode;
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::{
    layout::{Constraint, Layout},
    text::Line,
    widgets::{Block, Clear, Borders, Paragraph, Widget},
};
use tracing::info;

use crate::app::base::TargetInfo;
use crate::app::events::{ButtonRequest, Request};
use crate::app::ui::components::user_input::EInputMode;
use crate::app::ui::styles;
use crate::app::ui::views::base::view::{View, ViewStateController};
use crate::app::ui::views::button_view::{ButtonData, ButtonView};
use crate::app::ui::views::property_view::{PropertyData, PropertyView};

use super::base::view_state::ViewState;

#[derive(Debug, Clone, Default)]
pub struct PopupData {
    pub title: String,
    pub properties: Vec<PropertyData>,
	pub meta: Option<TargetInfo>
}

pub struct PopupView {
    data: PopupData,
    state: ViewState,
    properties: Vec<PropertyView>,
    buttons: Vec<ButtonView>
}

impl PopupView {
    pub fn new(data: &PopupData) -> Self {
        PopupView {
            data: data.clone(),
            ..Default::default()
        }
    }

    pub fn set_data(&mut self, data: &PopupData) {
        self.data = data.clone();
        self.properties.clear();
        for property_data in &data.properties {
            self.properties.push(PropertyView::new(property_data.clone()));
        }

        self.state.select(Some(0));
    }

    pub fn has_focused_property(&self) -> bool {
		match self.state.selected {
			Some(selected) => {
				match self.properties.get(selected) {
					Some(property) => {
						property.get_mode() == Some(EInputMode::Editing)
					}
					_=> {
						false
					}
				}
			}
			_=> {
				false
			}
		}
    }

	pub fn get_properties(&self) -> Vec<PropertyData> {
		self.properties.iter().map(|property| {property.data.clone()}).collect()
	}

    fn select_none(&mut self) {
        self.state.select(None);
    }

    fn select_next(&mut self) {
        self.state.select_next();
    }

    fn select_previous(&mut self) {
        self.state.select_previous();
    }
}

impl Default for PopupView {
    fn default() -> Self {
        let mut view = PopupView {
            data: PopupData::default(),
            state: ViewState::default(),
            properties: vec![],
            buttons: vec![
				ButtonView::new(ButtonData{
                    value: "Cancel".to_string(),
					click_action: Some(crate::app::events::ButtonRequest::Cancel)
                }),
                ButtonView::new(ButtonData {
                    value: "Confirm".to_string(),
					click_action: Some(crate::app::events::ButtonRequest::Confirm)
                })
            ]
        };

		return view;
    }
}

impl View for PopupView {
    type ViewData = PopupData;

    fn render_data(&mut self, area: Rect, buf: &mut Buffer, data: &Self::ViewData) {
		self.set_data(data);
		self.render(area, buf);
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        // let data = self.data.clone();
        // self.render_data(area, buf, &data);
    }

    fn render_frame(&mut self, frame: &mut Frame, area: Rect) {
        // Prefer the reference's compact width, retaining space on small terminals.
        let width = ((u32::from(area.width) * 62 / 100) as u16)
            .max(48)
            .min(area.width);
        let field_heights: Vec<u16> = self.properties.iter().map(|property| {
            if property.data.label.trim_end_matches(':').eq_ignore_ascii_case("description") {
                6
            } else {
                4
            }
        }).collect();
        let body_height = field_heights.iter().copied().sum::<u16>()
            + self.properties.len().saturating_sub(1) as u16;
        let height = body_height.saturating_add(10).min(area.height);
        let centered_area = area.centered(
            Constraint::Length(width),
            Constraint::Length(height),
        );

        frame.render_widget(Clear, centered_area);
        let popup_block = Block::bordered()
            .border_style(Style::default().fg(styles::POPUP_BORDER_COLOR))
            .style(Style::default().bg(styles::POPUP_BG).fg(styles::POPUP_TEXT_COLOR));
        let inner = popup_block.inner(centered_area);
        frame.render_widget(popup_block, centered_area);

        let [header, body, actions] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(4),
        ]).areas(inner);
        frame.render_widget(
            Paragraph::new(Line::styled(
                format!(" {}", self.data.title.to_uppercase()),
                Style::default().fg(styles::POPUP_TITLE_COLOR).add_modifier(Modifier::BOLD),
            )).block(Block::new().borders(Borders::BOTTOM)
                .border_style(Style::default().fg(styles::PROPERTY_BORDER_COLOR))),
            header,
        );

        let body = body.inner(ratatui::layout::Margin::new(1, 1));
        let mut constraints = Vec::new();
        for (index, height) in field_heights.iter().enumerate() {
            if index > 0 {
                constraints.push(Constraint::Length(1));
            }
            constraints.push(Constraint::Length(*height));
        }
        let property_areas = Layout::vertical(constraints).split(body);
        for (index, property) in self.properties.iter_mut().enumerate() {
            property.render_frame(frame, property_areas[index * 2]);
        }

        let action_block = Block::new().borders(Borders::TOP)
            .border_style(Style::default().fg(styles::PROPERTY_BORDER_COLOR));
        let action_inner = action_block.inner(actions);
        frame.render_widget(action_block, actions);
        let [hint, cancel, _gap, confirm] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(10),
            Constraint::Length(1),
            Constraint::Length(11),
        ]).areas(action_inner.inner(ratatui::layout::Margin::new(1, 0)));
        let hint_text = if self.has_focused_property() {
            "Enter/Esc finish editing"
        } else if hint.width >= 38 {
            "j/k navigate · i edit · Enter activate"
        } else {
            "j/k · i edit · Enter"
        };
        if hint.width >= hint_text.chars().count() as u16 + 1 {
            let hint_line = Rect { y: hint.y.saturating_add(1), height: hint.height.saturating_sub(1), ..hint };
            frame.render_widget(Paragraph::new(hint_text)
                .style(Style::default().fg(styles::DESCRIPTION_FG_COLOR)), hint_line);
        }
        for (button, button_area) in self.buttons.iter_mut().zip([cancel, confirm]) {
            button.render_frame(frame, button_area);
        }

    }
}

impl ViewStateController for PopupView {
    fn mark_selected(&mut self, is_selected: bool) {
    }

    fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<crate::app::events::Request, String> {
		match self.state.selected {
			Some(id) => {
				match self.properties.get_mut(id) {
					Some(view) => {
						let view_state = view as &mut dyn ViewStateController;
						let result = view_state.handle_key(key);
						if view.get_mode() == Some(EInputMode::Editing) {
							return result;
						}
					}
					_ => {
						let button_id = id - self.properties.len();
						match self.buttons.get_mut(button_id) {
							Some(view) => {
								let view_state = view as &mut dyn ViewStateController;
								let request = view_state.handle_key(key);
								match request.clone() {
									Ok(Request::Button(_)) => {
										return request;
									}
									_=> {
										
									}
								}
							}
							_=> {}
						}
					}
				}
			}
			_ => {}
		}

        match key.code {
            KeyCode::Char('h') => self.select_none(),
            KeyCode::Char('j') | KeyCode::Down | KeyCode::Right => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up | KeyCode::Left => self.select_previous(),
            _ => {
            }
        }

		let views_states: Vec<&mut dyn ViewStateController> = self.properties
			.iter_mut()
			.map(|view| view as &mut dyn ViewStateController)
			.chain(
				self.buttons
					.iter_mut()
					.map(|view| view as &mut dyn ViewStateController)
			)
			.collect();

		self.state.handle_secleceted(views_states);

        Ok(Request::Nothing)
    }

}
