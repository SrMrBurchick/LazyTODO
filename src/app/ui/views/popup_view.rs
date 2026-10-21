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
use crate::app::ui::views::base::view::{View, ViewState};
use crate::app::ui::views::button_view::{ButtonData, ButtonView};
use crate::app::ui::views::property_view::{PropertyData, PropertyView};

pub struct PropertyViewState {
    pub selected: Option<usize>,
}

pub struct NavigationViewState {
    pub selected: Option<usize>,
}

trait PopupViewStateSection {
    // fn bind_to_view(&mut self, view: &Box<dyn View<ViewData = Self::ViewStateData>>);
    fn select(&mut self, index: Option<usize>);
    fn select_next(&mut self);
    fn select_previous(&mut self);
    fn selected(&self) -> Option<usize>;
}

pub struct PopupViewState {
    pub selected: Option<usize>,
}

impl Default for PopupViewState {
	fn default() -> Self {
		PopupViewState {
			selected: None,
		}
	}
}


impl PopupViewState {
    pub const fn select(&mut self, index: Option<usize>) {
        self.selected = index;
    }

    pub fn select_next(&mut self) {
		let next = self.selected.map_or(0, |i| i.saturating_add(1));
		self.select(Some(next));
    }

    pub fn select_previous(&mut self) {
        let previous = self.selected.map_or(usize::MAX, |i| i.saturating_sub(1));
        self.select(Some(previous));
    }

	pub fn handle_secleceted(&mut self, views: Vec<&mut dyn ViewState>) {
		let size = views.len();

		if self.selected.is_some_and(|s| s >= size) {
			self.select(Some(size.saturating_sub(1)));
		}

		for (index, state) in views.into_iter().enumerate() {
			match self.selected {
				Some(id) => {
					state.mark_selected(id == index);
				},
				None => {
					state.mark_selected(false);
				}
			};
		
		}
	}
}

#[derive(Debug, Clone, Default)]
pub struct PopupData {
    pub title: String,
    pub properties: Vec<PropertyData>,
	pub meta: Option<TargetInfo>
}

pub struct PopupView {
    data: PopupData,
    state: PopupViewState,
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
            self.properties.push(PropertyView::new(property_data));
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
            state: PopupViewState::default(),
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
        let popup_block = Block::default()
            .borders(Borders::ALL)
            .title(
                Line::styled(
                    format!(" {} ", data.title.to_uppercase()),
                    Style::default()
                        .fg(styles::POPUP_TITLE_COLOR)
                        .add_modifier(Modifier::BOLD),
                )
            )
            .border_style(
                Style::default()
                    .fg(styles::POPUP_BORDER_COLOR)
            )
            .style(
                Style::default()
                    .bg(styles::POPUP_BG)
                    .fg(styles::POPUP_TEXT_COLOR)
            );

        let inner = popup_block.inner(area);
        popup_block.render(area, buf);

        let property_areas = Layout::horizontal(vec![
            Constraint::Fill(1);
            data.properties.len()
        ]).split(inner);

        for (property_data, property_area) in data.properties.iter().zip(property_areas.iter())
        {
            let mut property_view = PropertyView::new(property_data);
            property_view.render(*property_area, buf);
        }
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let data = self.data.clone();
        self.render_data(area, buf, &data);
    }

    fn render_frame(&mut self, frame: &mut Frame, area: Rect) {
        let centered_area = area.centered(
            Constraint::Percentage(60),
            Constraint::Percentage(60),
        );

        frame.render_widget(Clear, centered_area);

        let popup_block = Block::default()
            .borders(Borders::ALL)
            .title(
                Line::styled(
                    format!(" {} ", self.data.title.to_uppercase()),
                    Style::default()
                        .fg(styles::POPUP_TITLE_COLOR)
                        .add_modifier(Modifier::BOLD),
                )
            )
            .border_style(
                Style::default()
                    .fg(styles::SELECTED_SECTION_COLOR)
            )
            .style(
                Style::default()
                    .bg(styles::POPUP_BG)
                    .fg(styles::POPUP_TEXT_COLOR)
            );

        let inner = popup_block.inner(centered_area);
        popup_block.render(centered_area, frame.buffer_mut());

        let [data_area, buttons_area] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Percentage(20)
        ]).areas(inner);

		let views_states: Vec<&mut dyn ViewState> = self.properties
			.iter_mut()
			.map(|view| view as &mut dyn ViewState)
			.chain(
				self.buttons
					.iter_mut()
					.map(|view| view as &mut dyn ViewState)
			)
			.collect();

		self.state.handle_secleceted(views_states);

        let property_areas = Layout::vertical(vec![
            Constraint::Fill(1);
            self.properties.len()
        ]).split(data_area);

        for (property_view, property_area) in self.properties.iter_mut().zip(property_areas.iter()) {
            property_view.render_frame(frame, *property_area);
        }

        let button_areas = Layout::horizontal(vec![
            Constraint::Fill(1);
            self.properties.len()
        ]).split(buttons_area);

        for (button_view, button_area) in self.buttons.iter_mut().zip(button_areas.iter()) {
            button_view.render_frame(frame, *button_area);
        }

    }
}

impl ViewState for PopupView {
    fn mark_selected(&mut self, is_selected: bool) {
    }

    fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<crate::app::events::Request, String> {
		match self.state.selected {
			Some(id) => {
				match self.properties.get_mut(id) {
					Some(view) => {
						let view_state = view as &mut dyn ViewState;
						let result = view_state.handle_key(key);
						if view.get_mode() == Some(EInputMode::Editing) {
							return result;
						}
					}
					_ => {
						let button_id = id - self.properties.len();
						match self.buttons.get_mut(button_id) {
							Some(view) => {
								let view_state = view as &mut dyn ViewState;
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

        Ok(Request::Nothing)
    }

}
