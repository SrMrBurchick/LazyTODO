use ratatui::{layout::{Constraint, Layout}, symbols, widgets::{Block, Borders, Paragraph, Widget}};
use ratatui::text::Line;
use ratatui::style::{Color, Modifier, Style, Stylize};

use crate::app::ui::{sections::Inspectable, styles};

pub mod projects;
pub mod tasks;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Target {
    Project,
    Task,
    SubTask,
    All
}

// BigDICK TargetInfo
#[derive(Clone, Debug)]
pub enum TargetInfo {
    // Total / Completed
    Projects(i64, i64),
    // Id / Title / Description / Total / Completed / Progress
    Project(i64, String, String, i64, i64, i64),
    // Id / Title / Description / ProjectId / ProjectTitle / Total / Completed / Progress
    Task(i64, String, String, Option<i64>, Option<String>, i64, i64, i64),
    // Id / Title / Description / ParentTaskId / ParentTaskTitle / ProjectId / ProjectTitle
    SubTask(i64, String, String, i64, String, Option<i64>, Option<String>),
}

impl Inspectable for TargetInfo {
    fn get_name(&self) -> Option<String> {
        match self {
            TargetInfo::Projects(_, _) => {
                return Some("PROJECTS".to_string())
            },
            TargetInfo::Project(_, name, _, _, _, _) => {
                return Some(name.clone());
            },
            TargetInfo::Task(_, name, _, _, _, _, _, _) => {
                return Some(name.clone());
            },
            TargetInfo::SubTask(_, name, _, _, _, _, _) => {
                return Some(name.clone());
            },
            _ => {
            },
        }
        None
    }

    fn get_description(&self) -> Option<String> {
        match self {
            TargetInfo::Project(_, _, description, _, _, _) => {
                return Some(description.clone());
            },
            TargetInfo::Task(_, _, description, _, _, _, _, _) => {
                return Some(description.clone());
            },
            TargetInfo::SubTask(_, _, description, _, _, _, _) => {
                return Some(description.clone());
            },
            _ => {
            },
        }

        None
    }

    fn render_content(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        match self {
            TargetInfo::Projects(count, completed) => {
                let [count_area, completed_area] = Layout::horizontal([Constraint::Fill(1);2]).areas(area);
                let block = Block::new()
                    .borders(Borders::ALL)
                    .border_set(symbols::border::EMPTY)
                    .border_style(styles::TODO_HEADER_STYLE)
                    .bg(styles::NORMAL_ROW_BG);

                let mut block_content = block.inner(count_area);
                let mut data = Paragraph::new(vec![
                    Line::styled(count.to_string(), Style::default().fg(styles::TEXT_FG_COLOR).add_modifier(Modifier::BOLD)),
                    Line::styled("Total", Style::default().fg(styles::TEXT_FG_COLOR))

                ]).wrap(ratatui::widgets::Wrap { trim: true });
                data.render(block_content, buf);
                block_content = block.inner(completed_area);
                data = Paragraph::new(vec![
                    Line::styled(completed.to_string(), Style::default().fg(styles::TEXT_FG_COLOR).add_modifier(Modifier::BOLD)),
                    Line::styled("Completed", Style::default().fg(styles::TEXT_FG_COLOR))

                ]).wrap(ratatui::widgets::Wrap { trim: true });
                data.render(block_content, buf);
            },
            TargetInfo::Project(_, _, _, count, completed, progress) => {
                let [count_area, completed_area, progress_area] = Layout::horizontal([Constraint::Fill(1);3]).areas(area);
                let block = Block::new()
                    .borders(Borders::ALL)
                    .border_set(symbols::border::EMPTY)
                    .border_style(styles::TODO_HEADER_STYLE)
                    .bg(styles::NORMAL_ROW_BG);

                let mut block_content = block.inner(count_area);
                let mut data = Paragraph::new(vec![
                    Line::styled(count.to_string(), Style::default().fg(styles::TEXT_FG_COLOR).add_modifier(Modifier::BOLD)),
                    Line::styled("Total", Style::default().fg(styles::TEXT_FG_COLOR))

                ]).wrap(ratatui::widgets::Wrap { trim: true });
                data.render(block_content, buf);

                block_content = block.inner(completed_area);
                data = Paragraph::new(vec![
                    Line::styled(completed.to_string(), Style::default().fg(styles::TEXT_FG_COLOR).add_modifier(Modifier::BOLD)),
                    Line::styled("Completed", Style::default().fg(styles::TEXT_FG_COLOR))

                ]).wrap(ratatui::widgets::Wrap { trim: true });
                data.render(block_content, buf);

                block_content = block.inner(progress_area);
                data = Paragraph::new(vec![
                    Line::styled(format!("{progress} %"), Style::default().fg(styles::TEXT_FG_COLOR).add_modifier(Modifier::BOLD)),
                    Line::styled("Progress", Style::default().fg(styles::TEXT_FG_COLOR))

                ]).wrap(ratatui::widgets::Wrap { trim: true });
                data.render(block_content, buf);

            },
            TargetInfo::Task(_, _, _, _, _, count, completed, progress) => {
                let [count_area, completed_area, progress_area] = Layout::horizontal([Constraint::Fill(1);3]).areas(area);
                let block = Block::new()
                    .borders(Borders::ALL)
                    .border_set(symbols::border::EMPTY)
                    .border_style(styles::TODO_HEADER_STYLE)
                    .bg(styles::NORMAL_ROW_BG);

                let mut block_content = block.inner(count_area);
                let mut data = Paragraph::new(vec![
                    Line::styled(count.to_string(), Style::default().fg(styles::TEXT_FG_COLOR).add_modifier(Modifier::BOLD)),
                    Line::styled("Total", Style::default().fg(styles::TEXT_FG_COLOR))

                ]).wrap(ratatui::widgets::Wrap { trim: true });
                data.render(block_content, buf);

                block_content = block.inner(completed_area);
                data = Paragraph::new(vec![
                    Line::styled(completed.to_string(), Style::default().fg(styles::TEXT_FG_COLOR).add_modifier(Modifier::BOLD)),
                    Line::styled("Completed", Style::default().fg(styles::TEXT_FG_COLOR))

                ]).wrap(ratatui::widgets::Wrap { trim: true });
                data.render(block_content, buf);

                block_content = block.inner(progress_area);
                data = Paragraph::new(vec![
                    Line::styled(format!("{progress} %"), Style::default().fg(styles::TEXT_FG_COLOR).add_modifier(Modifier::BOLD)),
                    Line::styled("Progress", Style::default().fg(styles::TEXT_FG_COLOR))

                ]).wrap(ratatui::widgets::Wrap { trim: true });
                data.render(block_content, buf);
            }
            _ => {},
        }
    }
}
