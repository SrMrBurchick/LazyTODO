use crossterm::event::KeyCode;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::{
    layout::{Constraint, Layout},
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::app::events::{ButtonRequest, Request};
use crate::app::ui::styles;
use crate::app::ui::views::base::view::{View, ViewStateController};

#[derive(Clone, Default)]
pub struct ButtonData {
    pub value: String,
	pub click_action: Option<ButtonRequest>
}

type OnClickCallback = Box<dyn FnMut(&mut dyn ViewStateController)>;

pub struct ButtonView {
    data: ButtonData,
    selected: bool,
	on_click: Option<OnClickCallback>
}

impl ButtonView {
    pub fn new(data: ButtonData) -> Self {
        ButtonView {
            data: data.clone(),
            selected: false,
			on_click: None
        }
    }

	pub fn set_on_click(&mut self, callback: Option<OnClickCallback>) {
		self.on_click = callback;
	}

	fn on_click(&mut self) {
		match &mut self.on_click {
			Some(callback) => {
				// (callback)();
			}
			_ => {}
		}
	}
}

impl View for ButtonView {
    type ViewData = ButtonData;

    fn render_data(&mut self, area: Rect, buf: &mut Buffer, data: &Self::ViewData) {
        let block = Block::new()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(if self.selected {styles::PROPERTY_SELECTED_BORDER_COLOR} else {styles::PROPERTY_BORDER_COLOR}))
            .style(Style::default().bg(if self.selected { styles::POPUP_SELECTED_BG } else { styles::POPUP_BG }));

        let inner = block.inner(area);
        block.render(area, buf);
        Paragraph::new(
            Line::styled(data.value.as_str(), Style::default().fg(styles::POPUP_TEXT_COLOR))
        ).centered().render(inner, buf);

    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let data = self.data.clone();
        self.render_data(area, buf, &data);
    }

    fn render_frame(&mut self, frame: &mut ratatui::prelude::Frame, area: Rect) {
        self.render(area, frame.buffer_mut());
    }
}

impl ViewStateController for ButtonView {
    fn mark_selected(&mut self, is_selected: bool) {
        self.selected = is_selected;
    }

	fn is_seleceted(&self) -> bool {
		self.selected
	}

    fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<crate::app::events::Request, String> {
		match key.code {
			KeyCode::Enter => {
				return Ok(Request::Button(self.data.click_action.clone()));
			},
			_ => {
			}
		}

        Ok(Request::Nothing)
    }
}
