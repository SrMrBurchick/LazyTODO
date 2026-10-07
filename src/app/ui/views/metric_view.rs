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

#[derive(Clone, Default)]
pub struct MetricData {
    pub title: String,
    pub value : i64,
    pub highlighted: bool,
}

pub struct MetricView {
    data: MetricData,
}

impl MetricView {
    pub fn new(data: &MetricData) -> Self {
        MetricView {
            data: data.clone()
        }
    }
}

impl View for MetricView {
    type ViewData = MetricData;
    fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<crate::app::events::Request, String> {
        Ok(Request::Nothing)
    }

    fn render_data(&mut self, area: Rect, buf: &mut Buffer, data: &Self::ViewData) {
        let block = Block::new()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(styles::METRIC_BORDER_COLOR))
            .style(Style::default().bg(styles::METRIC_CARD_BG));

        let inner = block.inner(area);

        block.render(area, buf);

        let [label_area, value_area, _remaining] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0)
        ]).areas(inner);

        Paragraph::new(
            Line::styled(data.title.to_uppercase(), Style::default().fg(styles::METRIC_LABEL_COLOR))
        ).render(label_area, buf);

        let value_color = if data.highlighted {
            styles::METRIC_HIGHLIGHT_COLOR
        } else {
            styles::METRIC_VALUE_COLOR
        };

        Paragraph::new(
            Line::styled(data.value.to_string(), Style::default().fg(value_color).add_modifier(Modifier::BOLD))
        ).render(value_area, buf);
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let data = self.data.clone();
        self.render_data(area, buf, &data);
    }
}
