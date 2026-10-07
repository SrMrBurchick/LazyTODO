use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph, Widget, Wrap},
};

use crate::app::base::Target;
use crate::app::base::tasks::ETaskState;
use crate::app::events::Request;
use crate::app::ui::styles;
use crate::app::ui::views::base::view::View;
use crate::app::ui::views::metric_bar_view::{
    MetricBarData,
    MetricBarView,
};
use crate::app::ui::views::metric_view::{
    MetricData,
    MetricView,
};

#[derive(Default, Clone)]
pub struct InspectorData {
    pub path : Vec<String>,
    pub item_name : String,
    pub item_description : String,
    pub item_status : Option<ETaskState>,
    pub metrics: Vec<MetricData>,
    pub progress: Option<MetricBarData>,
    pub identity: Option<(Target, i64)>
}

pub struct InspectorView {
    data: Option<InspectorData>,
}

impl InspectorView {
    pub fn new() -> Self {
        InspectorView {
            data: None
        }
    }

    pub fn clean(&mut self) {
        self.data = None;
    }

    pub fn set_data(&mut self, data: &InspectorData) {
        self.data = Some(data.clone());
    }

    // pub fn select_none(&mut self) {
    //     self.state.select(None);
    // }
    //
    // pub fn select_next(&mut self) {
    //     self.state.select_next();
    // }
    //
    // pub fn select_previous(&mut self) {
    //     self.state.select_previous();
    // }
    //
    // pub const fn select_first(&mut self) {
    //     self.state.select_first();
    // }
    //
    // pub const fn select_last(&mut self) {
    //     self.state.select_last();
    // }
    //
    // pub const fn selected(&self) -> Option<usize> {
    //     self.state.selected()
    // }
    //
    // pub fn select(&mut self, id: Option<usize>) {
    //     match self.state.select(id) {
    //         _ => {},
    //     };
    // }

    fn handle_navigation(&mut self, key: crossterm::event::KeyEvent)  -> Result<crate::app::events::Request, String> {
        // match key.code {
        //     KeyCode::Char('h') => self.select_none(),
        //     KeyCode::Char('j') | KeyCode::Down => self.select_next(),
        //     KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
        //     KeyCode::Char('g') | KeyCode::Home => self.select_first(),
        //     KeyCode::Char('G') | KeyCode::End => self.select_last(),
        //     _ => {
        //     }
        // }


        Ok(Request::Nothing)
    }
}

impl View for InspectorView {
    type ViewData = InspectorData;
    fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<crate::app::events::Request, String> {
        // match self.handle_navigation(key) {
        //
        //     Ok(request) => {
        //         if request != Request::Nothing {
        //             return Ok(request);
        //         }
        //     },
        //     Err(_) => {},
        // }
        //
        // match self.state.selected() {
        //     Some(id) => {
        //         match self.data.get_mut(id) {
        //             Some(item) => {
        //                 return item.handle_key(key);
        //             },
        //             None => {},
        //         }
        //     },
        //     None => {},
        // }

        Ok(Request::Nothing)
    }

    fn render_data(&mut self, area: Rect, buf: &mut Buffer, data: &Self::ViewData) {
        let block = Block::new()
            .title(Line::styled(
                " INSPECT ",
                Style::default().fg(styles::TODO_HEADER_STYLE.fg.unwrap_or(styles::TEXT_FG_COLOR)))
            )
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(styles::METRIC_BORDER_COLOR))
            .style(Style::default().bg(styles::NORMAL_ROW_BG))
            .padding(Padding::horizontal(1));

        let inner = block.inner(area);
        block.render(area, buf);

        let metrics_height = if data.metrics.is_empty() { 0 } else { 4 };

        let progress_height = if data.progress.is_some() { 4 } else { 0 };

        let [
            breadcrumb_area,
            header_area,
            type_area,
            description_area,
            separator_area,
            metrics_area,
            progress_area,
            _remaining,
        ] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(metrics_height),
            Constraint::Length(progress_height),
            Constraint::Min(0),
        ]).areas(inner);

        //
        // Breadcrumb
        //

        let mut breadcrumb_spans = Vec::new();

        for (index, part) in data
            .path
            .iter()
            .chain(std::iter::once(&data.item_name))
            .enumerate()
        {
            if index != 0 {
                breadcrumb_spans.push(Span::styled(
                    " / ",
                    Style::default().fg(styles::METRIC_BORDER_COLOR)
                ));
            }

            breadcrumb_spans.push(Span::styled(
                part.as_str(),
                Style::default().fg(styles::METRIC_LABEL_COLOR),
            ));
        }

        Paragraph::new(Line::from(breadcrumb_spans))
            .wrap(Wrap { trim: true })
            .render(breadcrumb_area, buf);

        //
        // Header: name + ID
        //
        let [
            name_area,
            id_area,
        ] = Layout::horizontal([
            Constraint::Min(0),
            Constraint::Length(8),
        ]).areas(header_area);

        Paragraph::new(data.item_name.as_str())
            .style(Style::default().fg(styles::METRIC_VALUE_COLOR).add_modifier(Modifier::BOLD))
            .render(name_area, buf);

        if let Some((_, id)) = &data.identity {
            Paragraph::new(format!("#{id}"))
                .alignment(Alignment::Right)
                .style(Style::default().fg(styles::METRIC_LABEL_COLOR))
                .render(id_area, buf);
        }

        //
        // Type + status
        //
        let target_name = match &data.identity {
            Some((Target::Project, _)) => "PROJECT",
            Some((Target::Task, _)) => "TASK",
            Some((Target::SubTask, _)) => "SUBTASK",
            _ => "SPACE",
        };

        let status_name = match &data.item_status {
            Some(ETaskState::Todo) => Some("TODO"),
            Some(ETaskState::InProgress) => Some("IN PROGRESS"),
            Some(ETaskState::Completed) => Some("COMPLETED"),
            None => None,
        };

        let type_text = match status_name {
            Some(status) => {
                format!("{target_name} · {status}")
            }

            None => {
                target_name.to_string()
            }
        };

        Paragraph::new(type_text)
            .style(Style::default().fg(styles::TODO_HEADER_STYLE.fg.unwrap_or(styles::TEXT_FG_COLOR)))
            .render(type_area, buf);

        //
        // Description
        //
        Paragraph::new(data.item_description.as_str())
            .style(Style::default().fg(styles::TEXT_FG_COLOR))
            .wrap(Wrap { trim: true })
            .render(description_area, buf);

        //
        // Separator
        //
        Block::new()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(styles::METRIC_BORDER_COLOR))
            .render(separator_area, buf);

        //
        // Metrics
        //
        if !data.metrics.is_empty() {
            let metric_areas = Layout::horizontal(vec![
                Constraint::Fill(1);
                data.metrics.len()
            ]).split(metrics_area);

            for (metric, metric_area) in data.metrics.iter().zip(metric_areas.iter())
            {
                let mut metric_view = MetricView::new(metric);

                metric_view.render(*metric_area, buf);
            }
        }

        //
        // Progress
        //
        if let Some(progress) = &data.progress {
            let mut progress_view = MetricBarView::new(progress);

            progress_view.render(progress_area, buf);
        }
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        match &self.data {
            Some(data) => {
                let render_data = data.clone();
                self.render_data(area, buf, &render_data);
            },
            None => {
                let block = Block::new()
                    .title(Line::styled(
                        " INSPECT ",
                        Style::default().fg(styles::TODO_HEADER_STYLE.fg.unwrap_or(styles::TEXT_FG_COLOR)))
                    )
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(styles::METRIC_BORDER_COLOR))
                    .style(Style::default().bg(styles::NORMAL_ROW_BG))
                    .padding(Padding::horizontal(1));

                block.render(area, buf);
            },
        }
    }
}
