use crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{
    Block, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph,
    StatefulWidget, Widget, Wrap,
};
use ratatui::symbols;
use tracing::warn;

use crate::app::base::{Target, TargetInfo};
use crate::app::base::projects::{Project, projects_to_widget_list};
use crate::app::base::tasks::{SubTask, Task, sub_tasks_to_widget_list, tasks_to_widget_list};
use crate::app::events::event_listener::EventsListener;
use crate::app::events::{DatabaseResponse, Request, SelectionResponse};
use crate::app::ui::sections::{ESectionId, Inspectable};
use crate::app::ui::views::base::view::View;
use crate::app::{events::Response, ui::{base::widget_list::WidgetListItem, sections::base::section::Section, styles, views::list_view::ListView}};

pub struct InspectorSection {
    view: ListView,
    target_info: Option<TargetInfo>
}

impl InspectorSection {
    pub fn new() -> Self {
        InspectorSection {
            view: ListView::new("Inspector"),
            target_info: None
        }
    }
}

impl Section for InspectorSection {
    fn initialize(&mut self) {
    }

    fn reset(&mut self) {
        self.view.clean();
    }

    fn get_id(&self) -> Option<ESectionId> {
        Some(ESectionId::Inspector)
    }

    fn handle_response(&mut self, response: &Response) {
        match response {
            Response::Database(database_response) => {
                match database_response {
                    DatabaseResponse::Info(id, info) => {
                        self.target_info = Some(info.clone());
                    },
                    _ => {
                    },
                }
            },
            _ => {
            },
        }
    }

    fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<Request, String> {
        self.view.handle_key(key)
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("INSPECT").centered())
            .borders(Borders::ALL)
            .border_set(symbols::border::EMPTY)
            .border_style(styles::TODO_HEADER_STYLE)
            .bg(styles::NORMAL_ROW_BG);

        let inner = block.inner(area);
        block.render(area, buf);

        let [
            header_area,
            info_area,
            bottom_block
        ] = Layout::vertical([
            Constraint::Percentage(10),
            Constraint::Percentage(40),
            Constraint::Percentage(20),
        ])
        .areas(inner);

        match &self.target_info {
            Some(info) => {
                let mut name: String = String::default();
                let mut description: String = String::default();
                let header: Paragraph<'_>;
                match info.get_name() {
                    Some(info_name) => {
                        name = info_name;
                    },
                    None => {},
                }

                match info.get_description() {
                    Some(info_description) => {
                        description = info_description;
                    },
                    None => {},
                }

                if name.is_empty() {
                    warn!("Invalid name for target");
                    return;
                }

                if description.is_empty() {
                    header = Paragraph::new(vec![
                        Line::styled(
                            "Path",
                            Style::default()
                                .fg(styles::TEXT_FG_COLOR)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Line::styled(
                            name,
                            styles::TODO_HEADER_STYLE,
                        ),
                    ]).wrap(Wrap { trim: true });

                } else {
                    header = Paragraph::new(vec![
                        Line::styled(
                            "Path",
                            Style::default()
                                .fg(styles::TEXT_FG_COLOR)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Line::styled(
                            name,
                            styles::TODO_HEADER_STYLE,
                        ),
                        Line::raw(""),
                        Line::styled(
                            description,
                            Style::default().fg(styles::TEXT_FG_COLOR),
                        ),
                    ]).wrap(Wrap { trim: true });
                }

                header.render(header_area, buf);
                info.render_content(info_area, buf);
            },
            None => {},
        }

        // // Header
        //
        // // KPI
        // let [tasks_area, done_area, progress_area] = Layout::horizontal([
        //     Constraint::Ratio(1, 3),
        //     Constraint::Ratio(1, 3),
        //     Constraint::Ratio(1, 3),
        // ])
        // .areas(kpi_area);
        //
        // let tasks = Paragraph::new(vec![
        //     Line::styled(
        //         "11",
        //         styles::TODO_HEADER_STYLE
        //             .add_modifier(Modifier::BOLD),
        //     ),
        //     Line::styled(
        //         "tasks",
        //         Style::default()
        //             .fg(styles::COMPLETED_TEXT_FG_COLOR),
        //     ),
        // ]);
        //
        // let done = Paragraph::new(vec![
        //     Line::styled(
        //         "4",
        //         styles::TODO_HEADER_STYLE
        //             .add_modifier(Modifier::BOLD),
        //     ),
        //     Line::styled(
        //         "done",
        //         Style::default()
        //             .fg(styles::COMPLETED_TEXT_FG_COLOR),
        //     ),
        // ]);
        //
        // let progress = Paragraph::new(vec![
        //     Line::styled(
        //         "36%",
        //         styles::TODO_HEADER_STYLE
        //             .add_modifier(Modifier::BOLD),
        //     ),
        //     Line::styled(
        //         "progress",
        //         Style::default()
        //             .fg(styles::COMPLETED_TEXT_FG_COLOR),
        //     ),
        // ]);
        //
        // tasks.render(tasks_area, buf);
        // done.render(done_area, buf);
        // progress.render(progress_area, buf);
        //
        // // Separator
        // Paragraph::new("────────────────────────────────")
        //     .style(
        //         Style::default()
        //             .fg(styles::COMPLETED_TEXT_FG_COLOR),
        //     )
        //     .render(separator_area, buf);
        //
        // // Next
        // let next = Paragraph::new(vec![
        //     Line::styled(
        //         "NEXT",
        //         styles::TODO_HEADER_STYLE
        //             .add_modifier(Modifier::BOLD),
        //     ),
        //     Line::raw(""),
        //     Line::styled(
        //         "→ Switch Projects / Tasks / SubTasks",
        //         Style::default()
        //             .fg(styles::TEXT_FG_COLOR),
        //     ),
        // ])
        // .wrap(Wrap { trim: true });
        //
        // next.render(next_area, buf);
        //
        // // Metadata
        // Paragraph::new("updated 23:04 · local")
        //     .style(
        //         Style::default()
        //             .fg(styles::COMPLETED_TEXT_FG_COLOR),
        //     )
        //     .render(metadata_area, buf);

        // self.view.render(area, buf);
    }
}
