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

use crate::app::base::{Target, TargetInfo};
use crate::app::base::projects::{Project, projects_to_widget_list};
use crate::app::base::tasks::{SubTask, Task, sub_tasks_to_widget_list, tasks_to_widget_list};
use crate::app::events::event_listener::EventsListener;
use crate::app::events::{DatabaseResponse, Request, SelectionResponse};
use crate::app::ui::sections::ESectionId;
use crate::app::ui::views::base::view::View;
use crate::app::{events::Response, ui::{base::widget_list::WidgetListItem, sections::base::section::Section, styles, views::list_view::ListView}};

pub struct InspectorSection {
    view: ListView,
    selection_response: Option<SelectionResponse>
}

impl InspectorSection {
    pub fn new() -> Self {
        InspectorSection {
            view: ListView::new("Inspector"),
            selection_response: None
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
                        match info {
                            TargetInfo::Projects(total, completed) => {

                            },
                            _ => {},
                        }

                    },
                    _ => {
                    },
                }
            },
            Response::Selection(response) => {
                self.selection_response = Some(response.clone());
            }
            _ => {
            },
        }
    }

    fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<Request, String> {
        self.view.handle_key(key)
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        match &self.selection_response {
            Some(response) => {
                match response {
                    SelectionResponse::Selected(target, first, second) => {
                        let mut view_data: Vec<Box<dyn WidgetListItem>> = vec![];
                        match target {
                            Target::Task => {
                                match first {
                                    Some(project_id) => {
                                        for project in self.projects.iter() {
                                            if project.id == *project_id {
                                                for task in project.tasks.iter() {
                                                    match second {
                                                        Some(task_id) => {
                                                            if task.id == *task_id {
                                                                // view_data.push(Box::new(task.clone()) as Box<dyn WidgetListItem>);
                                                            }
                                                        },
                                                        None => {},
                                                    }
                                                }
                                            }
                                        }
                                    },
                                    None => {
                                        for task in self.tasks.iter() {
                                            match second {
                                                Some(task_id) => {
                                                    if task.id == *task_id {
                                                        // view_data.push(Box::new(task.clone()) as Box<dyn WidgetListItem>);
                                                    }
                                                },
                                                None => {},
                                            }
                                        }

                                    },
                                }
                            },
                            Target::SubTask => {
                                match second {
                                    Some(task_id) => {
                                        for task in self.subtasks.iter() {
                                            if task.id == *task_id {
                                                // view_data.push(Box::new(task.clone()) as Box<dyn WidgetListItem>);
                                            }
                                        }

                                    },
                                    None => {

                                    },
                                }
                            },
                            Target::Project => {
                                match second {
                                    Some(task_id) => {
                                        for task in self.projects.iter() {
                                            if task.id == *task_id {
                                                // view_data.push(Box::new(task.clone()) as Box<dyn WidgetListItem>);
                                            }
                                        }
                                    },
                                    None => {

                                    },
                                }
                            },
                            _ => {},
                        }

                        if !view_data.is_empty() {
                            // self.view.set_items(view_data);
                        }
                    }
                    _ => {},
                }
            },
            None => {},
        }

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
            separator_area,
            kpi_area,
            next_area,
            metadata_area,
            _remaining,
        ] = Layout::vertical([
            Constraint::Length(5),
            Constraint::Length(1),
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(2),
            Constraint::Min(0),
        ])
        .areas(inner);

        // Header
        let header = Paragraph::new(vec![
            Line::styled(
                "◈ LazyTODO",
                Style::default()
                    .fg(styles::TEXT_FG_COLOR)
                    .add_modifier(Modifier::BOLD),
            ),
            Line::styled(
                "PROJECT",
                styles::TODO_HEADER_STYLE,
            ),
            Line::raw(""),
            Line::styled(
                "TUI task manager / local-first",
                Style::default().fg(styles::TEXT_FG_COLOR),
            ),
        ])
        .wrap(Wrap { trim: true });

        header.render(header_area, buf);

        // KPI
        let [tasks_area, done_area, progress_area] = Layout::horizontal([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .areas(kpi_area);

        let tasks = Paragraph::new(vec![
            Line::styled(
                "11",
                styles::TODO_HEADER_STYLE
                    .add_modifier(Modifier::BOLD),
            ),
            Line::styled(
                "tasks",
                Style::default()
                    .fg(styles::COMPLETED_TEXT_FG_COLOR),
            ),
        ]);

        let done = Paragraph::new(vec![
            Line::styled(
                "4",
                styles::TODO_HEADER_STYLE
                    .add_modifier(Modifier::BOLD),
            ),
            Line::styled(
                "done",
                Style::default()
                    .fg(styles::COMPLETED_TEXT_FG_COLOR),
            ),
        ]);

        let progress = Paragraph::new(vec![
            Line::styled(
                "36%",
                styles::TODO_HEADER_STYLE
                    .add_modifier(Modifier::BOLD),
            ),
            Line::styled(
                "progress",
                Style::default()
                    .fg(styles::COMPLETED_TEXT_FG_COLOR),
            ),
        ]);

        tasks.render(tasks_area, buf);
        done.render(done_area, buf);
        progress.render(progress_area, buf);

        // Separator
        Paragraph::new("────────────────────────────────")
            .style(
                Style::default()
                    .fg(styles::COMPLETED_TEXT_FG_COLOR),
            )
            .render(separator_area, buf);

        // Next
        let next = Paragraph::new(vec![
            Line::styled(
                "NEXT",
                styles::TODO_HEADER_STYLE
                    .add_modifier(Modifier::BOLD),
            ),
            Line::raw(""),
            Line::styled(
                "→ Switch Projects / Tasks / SubTasks",
                Style::default()
                    .fg(styles::TEXT_FG_COLOR),
            ),
        ])
        .wrap(Wrap { trim: true });

        next.render(next_area, buf);

        // Metadata
        Paragraph::new("updated 23:04 · local")
            .style(
                Style::default()
                    .fg(styles::COMPLETED_TEXT_FG_COLOR),
            )
            .render(metadata_area, buf);

        // self.view.render(area, buf);
    }
}
