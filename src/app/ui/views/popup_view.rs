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

use crate::app::events::Request;
use crate::app::ui::styles;
use crate::app::ui::views::base::view::View;
use crate::app::ui::views::property_view::{PropertyData, PropertyView};

#[derive(Debug, Clone, Default)]
pub struct PopupData {
    pub title: String,
    pub properties: Vec<PropertyData>
}

pub struct PopupView {
    data: PopupData,
}

impl PopupView {
    pub fn new(data: &PopupData) -> Self {
        PopupView {
            data: data.clone()
        }
    }

    pub fn set_data(&mut self, data: &PopupData) {
        self.data = data.clone();
    }
}

impl Default for PopupView {
    fn default() -> Self {
        PopupView { data: PopupData::default() }
    }
}

impl View for PopupView {
    type ViewData = PopupData;
    fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<crate::app::events::Request, String> {
        Ok(Request::Nothing)
    }

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
            Constraint::Percentage(40),
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
                    .fg(styles::POPUP_BORDER_COLOR)
            )
            .style(
                Style::default()
                    .bg(styles::POPUP_BG)
                    .fg(styles::POPUP_TEXT_COLOR)
            );

        let inner = popup_block.inner(centered_area);
        popup_block.render(centered_area, frame.buffer_mut());

        info!("Properties count: {}", self.data.properties.len());
        let property_areas = Layout::horizontal(vec![
            Constraint::Fill(1);
            self.data.properties.len()
        ]).split(inner);

        for (property_data, property_area) in self.data.properties.iter().zip(property_areas.iter())
        {
            let mut property_view = PropertyView::new(property_data);

            property_view.render_frame(frame, *property_area);
        }
    }

    fn mark_selected(&mut self, is_selected: bool) {
    }
}
