use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::{
    layout::{Alignment, Constraint, Layout},
    text::Line,
    widgets::{
        Block,
        Borders,
        Gauge,
        Padding,
        Paragraph,
        Widget,
    },
};

use crate::app::events::Request;
use crate::app::ui::styles;
use crate::app::ui::views::base::view::View;

#[derive(Clone)]
pub struct MetricBarData {
    pub title: String,
    pub value : i64,
    pub highlighted: bool,
    pub min : i64,
    pub max : i64
}

impl Default for MetricBarData {
    fn default() -> Self {
        MetricBarData {
            title: String::default(),
            value: 0,
            highlighted: false,
            min: 0,
            max: 100
        }
    }
}


impl MetricBarData {
    pub fn get_percentage(&self) -> f64 {
        if self.max <= self.min {
            return 0.0;
        }

        let value = self.value.clamp(self.min, self.max);
        return (value - self.min) as f64 / (self.max - self.min) as f64;
    }
}


pub struct MetricBarView {
    data: MetricBarData,
}

impl MetricBarView {
    pub fn new(data: &MetricBarData) -> Self {
        MetricBarView {
            data: data.clone()
        }
    }
}

impl View for MetricBarView {
    type ViewData = MetricBarData;
    fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<crate::app::events::Request, String> {
        Ok(Request::Nothing)
    }

    fn render_data(&mut self, area: Rect, buf: &mut Buffer, data: &Self::ViewData) {
        let block = Block::new()
            .borders(Borders::ALL)
            .border_style(
                Style::default()
                    .fg(styles::METRIC_BORDER_COLOR)
            )
            .style(
                Style::default()
                    .bg(styles::METRIC_CARD_BG)
            )
            .padding(
                Padding::horizontal(1)
            );

        let inner = block.inner(area);

        block.render(area, buf);

        let [header_area, bar_area, remaining] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .areas(inner);

        // Title + percentage
        let [title_area, percentage_area] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(6),
        ])
        .areas(header_area);

        Paragraph::new(
            Line::styled(data.title.to_uppercase(), Style::default().fg(styles::METRIC_LABEL_COLOR))
        ).render(title_area, buf);

        let percentage = data.get_percentage();

        Paragraph::new(format!("{:.0}%", percentage * 100.0))
        .alignment(Alignment::Right)
        .style(Style::default().fg(styles::METRIC_LABEL_COLOR))
        .render(percentage_area, buf);

        // Progress bar
        let bar_color = if data.highlighted {
            styles::METRIC_HIGHLIGHT_COLOR
        } else {
            styles::METRIC_VALUE_COLOR
        };

        Gauge::default()
            .ratio(percentage)
            .label("")
            .gauge_style(Style::default().fg(bar_color).bg(styles::PROGRESS_TRACK_BG))
            .render(bar_area, buf);
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let data = self.data.clone();
        self.render_data(area, buf, &data);
    }
}
