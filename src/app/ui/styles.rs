use ratatui::style::{Modifier, Style};
use ratatui::style::palette::tailwind::{BLUE, GREEN, SLATE};
use ratatui::{style::Color, text::Line, widgets::ListItem};

use crate::app::base::projects::Project;
use crate::app::base::tasks::{ETaskState, SubTask, Task};

pub const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
pub const NORMAL_ROW_BG: Color = SLATE.c950;
pub const ALT_ROW_BG_COLOR: Color = SLATE.c900;
pub const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
pub const TEXT_FG_COLOR: Color = SLATE.c200;
pub const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;

impl From<&Project> for ListItem<'_> {
    fn from(value: &Project) -> Self {
        ListItem::new(Line::styled(format!("{value}"), TEXT_FG_COLOR))
    }
}

impl From<&Task> for ListItem<'_> {
    fn from(value: &Task) -> Self {
        let mut color = TEXT_FG_COLOR;
        let mut prefix = "☐";
        match value.state {
            ETaskState::Completed => {
                prefix = "✓";
                color = COMPLETED_TEXT_FG_COLOR;
            }
            _ => {},
        }

        ListItem::new(Line::styled(format!("{} {} - {}", prefix, value.title, value.description), color))
    }
}

impl From<&SubTask> for ListItem<'_> {
    fn from(value: &SubTask) -> Self {
        let mut color = TEXT_FG_COLOR;
        let mut prefix = "☐";
        match value.state {
            ETaskState::Completed => {
                prefix = "✓";
                color = COMPLETED_TEXT_FG_COLOR;
            }
            _ => {},
        }

        ListItem::new(Line::styled(format!("{} {value}", prefix), color))
    }
}
