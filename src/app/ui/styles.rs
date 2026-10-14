use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::ListItem,
};

use crate::app::base::projects::Project;
use crate::app::base::tasks::{ETaskState, SubTask, Task};

// Main UI
pub const TODO_HEADER_STYLE: Style = Style::new()
    .fg(Color::Rgb(145, 174, 190))
    .bg(Color::Rgb(12, 15, 18));

pub const SELECTED_STYLE: Style = Style::new()
    .fg(Color::Rgb(218, 228, 234))
    .bg(Color::Rgb(32, 43, 51))
    .add_modifier(Modifier::BOLD);

// Selected section
pub const SELECTED_SECTION_COLOR: Color =
    Color::Rgb(72, 102, 118);

// Rows
pub const NORMAL_ROW_BG: Color =
    Color::Rgb(15, 18, 22);

pub const ALT_ROW_BG_COLOR: Color =
    Color::Rgb(18, 22, 27);

// Text hierarchy
pub const TITLE_FG_COLOR: Color =
    Color::Rgb(196, 207, 214);

pub const DESCRIPTION_FG_COLOR: Color =
    Color::Rgb(112, 126, 136);

pub const COMPLETED_TEXT_FG_COLOR: Color =
    Color::Rgb(120, 136, 145);

// Status marks
pub const TODO_MARK_FG_COLOR: Color =
    Color::Rgb(104, 126, 137);

pub const COMPLETED_MARK_FG_COLOR: Color =
    Color::Rgb(124, 170, 150);

pub const IN_PROGRESS_MARK_FG_COLOR: Color =
    Color::Rgb(201, 179, 122);

// Inspector / metrics
pub const METRIC_CARD_BG: Color =
    Color::Rgb(17, 24, 29);

pub const METRIC_BORDER_COLOR: Color =
    Color::Rgb(30, 42, 49);

pub const METRIC_LABEL_COLOR: Color =
    Color::Rgb(104, 126, 137);

pub const METRIC_VALUE_COLOR: Color =
    Color::Rgb(218, 228, 234);

pub const METRIC_HIGHLIGHT_COLOR: Color =
    Color::Rgb(124, 247, 196);

pub const PROGRESS_TRACK_BG: Color =
    Color::Rgb(23, 33, 40);

// Property / input
pub const PROPERTY_BG: Color =
    Color::Rgb(14, 19, 23);

pub const PROPERTY_BORDER_COLOR: Color =
    Color::Rgb(30, 42, 49);

pub const PROPERTY_SELECTED_BORDER_COLOR: Color =
    Color::Rgb(72, 102, 118);

pub const PROPERTY_EDITING_BORDER_COLOR: Color =
    Color::Rgb(201, 179, 122);

pub const PROPERTY_LABEL_COLOR: Color =
    Color::Rgb(104, 126, 137);

pub const PROPERTY_VALUE_COLOR: Color =
    Color::Rgb(196, 207, 214);

pub const PROPERTY_READ_ONLY_COLOR: Color =
    Color::Rgb(88, 103, 112);

// Popup
pub const POPUP_BG: Color =
    Color::Rgb(12, 16, 20);

pub const POPUP_BORDER_COLOR: Color =
    Color::Rgb(54, 74, 86);

pub const POPUP_TITLE_COLOR: Color =
    Color::Rgb(145, 174, 190);

pub const POPUP_TEXT_COLOR: Color =
    Color::Rgb(196, 207, 214);

impl<'a> From<&'a Project> for ListItem<'a> {
    fn from(value: &'a Project) -> Self {
        ListItem::new(
            Line::styled(
                value.title.as_str(),
                Style::default()
                    .fg(TITLE_FG_COLOR)
                    .add_modifier(Modifier::BOLD),
            )
        )
    }
}

impl<'a> From<&'a Task> for ListItem<'a> {
    fn from(value: &'a Task) -> Self {
        let (prefix, prefix_color, title_color) = match value.state {
            ETaskState::Completed => (
                "✓",
                COMPLETED_MARK_FG_COLOR,
                COMPLETED_TEXT_FG_COLOR,
            ),
            ETaskState::InProgress => (
                "◐",
                IN_PROGRESS_MARK_FG_COLOR,
                TITLE_FG_COLOR,
            ),
            _ => (
                "☐",
                TODO_MARK_FG_COLOR,
                TITLE_FG_COLOR,
            ),
        };

        ListItem::new(vec![
            Line::from(vec![
                Span::styled(
                    format!("{prefix} "),
                    Style::default().fg(prefix_color),
                ),

                Span::styled(
                    value.title.as_str(),
                    Style::default()
                        .fg(title_color)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),

            Line::styled(
                format!("   {}", value.description),
                Style::default().fg(DESCRIPTION_FG_COLOR),
            ),
        ])
    }
}

impl<'a> From<&'a SubTask> for ListItem<'a> {
    fn from(value: &'a SubTask) -> Self {
        let (prefix, prefix_color, title_color) = match value.state {
            ETaskState::Completed => (
                "✓",
                COMPLETED_MARK_FG_COLOR,
                COMPLETED_TEXT_FG_COLOR,
            ),
            ETaskState::InProgress => (
                "◐",
                IN_PROGRESS_MARK_FG_COLOR,
                TITLE_FG_COLOR,
            ),
            _ => (
                "☐",
                TODO_MARK_FG_COLOR,
                TITLE_FG_COLOR,
            ),
        };

        ListItem::new(
            Line::from(vec![
                Span::styled(
                    format!("{prefix} "),
                    Style::default().fg(prefix_color),
                ),

                Span::styled(
                    value.title.as_str(),
                    Style::default()
                        .fg(title_color)
                        .add_modifier(Modifier::BOLD),
                ),
            ])
        )
    }
}
