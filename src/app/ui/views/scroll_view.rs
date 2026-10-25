use crossterm::event::KeyCode;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::symbols::scrollbar;
use ratatui::{
    layout::{Constraint, Layout},
    text::Line,
    widgets::{Block, Scrollbar, ScrollbarState, Borders, Padding, Paragraph, Widget},
};

use crate::app::events::Request;
use crate::app::ui::styles;
use crate::app::ui::views::base::view::{View, ViewStateController};

#[derive(Debug, Clone, Default)]
pub struct ScrollData {
    /// Total amount of visual lines in content.
    pub content_length: usize,

    /// Amount of lines currently visible inside viewport.
    pub viewport_length: usize,

    /// Current first visible line.
    pub offset: usize,

    /// Line that must remain visible.
    /// For PropertyView this is the visual cursor line.
    pub tracked_position: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum EScrollOrientation {
	Horizontal,
	Vertical
}

#[derive(Debug, Clone, Default)]
pub struct ScrollView {
    data: ScrollData,
	state: ScrollbarState,
	orientation: Option<EScrollOrientation>,
	scroll: usize
}

impl ScrollView {
    pub fn new() -> Self {
        ScrollView {
            data: ScrollData::default(),
			orientation: None,
			state: ScrollbarState::default(),
			scroll: 0
        }
    }

	pub fn update(&mut self, content_length: usize, viewport_length: usize) {
		self.data.content_length = content_length;
		self.data.viewport_length = viewport_length;

        self.clamp_offset();
        self.update_state();	
	}

    pub fn track(&mut self, position: usize) {
        self.data.tracked_position = Some(position);

        self.ensure_visible(position);
        self.update_state();
    }

    pub fn clear_tracking(&mut self) {
        self.data.tracked_position = None;
    }

    pub fn scroll_forward(&mut self) {
        let max_offset = self.max_offset();

        self.data.offset = self
            .data
            .offset
            .saturating_add(1)
            .min(max_offset);

        self.update_state();
    }

    pub fn scroll_back(&mut self) {
        self.data.offset = self.data.offset.saturating_sub(1);

        self.update_state();
    }

    pub fn ensure_visible(&mut self, position: usize) {
        if self.data.viewport_length == 0 {
            return;
        }

        if position < self.data.offset {
            self.data.offset = position;
        } else if position >= self.data.offset + self.data.viewport_length {
            self.data.offset = position.saturating_add(1).saturating_sub(self.data.viewport_length);
        }

        self.clamp_offset();
    }

    fn max_offset(&self) -> usize {
        self.data.content_length.saturating_sub(self.data.viewport_length)
    }

    fn clamp_offset(&mut self) {
        self.data.offset = self.data.offset.min(self.max_offset());
    }

    fn update_state(&mut self) {
        self.state = ScrollbarState::default()
            .content_length(self.data.content_length)
            .viewport_content_length(self.data.viewport_length)
            .position(self.data.offset);
    }

    pub fn get_scroll(&self) -> usize {
        self.data.offset
    }

    pub fn get_visible_position(&self, position: usize) -> usize {
        position.saturating_sub(self.data.offset)
    }

    pub fn is_scrollable(&self) -> bool {
        self.data.content_length > self.data.viewport_length
    }	
}

impl View for ScrollView {
    type ViewData = ScrollData;

    fn render_data(&mut self, area: Rect, buf: &mut Buffer, data: &Self::ViewData) {
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
    }

	fn render_frame(&mut self, frame: &mut ratatui::Frame, area: Rect) {
		frame.render_stateful_widget(
			Scrollbar::new(ratatui::widgets::ScrollbarOrientation::VerticalRight)
                .symbols(scrollbar::VERTICAL)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓"))
                .end_symbol(None),

			area.inner(ratatui::layout::Margin {
				vertical: 1,
				horizontal: 0
			}),
			&mut self.state
		);
	}
}

impl ViewStateController for ScrollView {
    fn mark_selected(&mut self, is_selected: bool) {
    }

	fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<crate::app::events::Request, String> {
		match key.code {
			KeyCode::Down => self.scroll_forward(),
			KeyCode::Up => self.scroll_back(),
			_ => {}
		}

        Ok(Request::Nothing)
    }
}
