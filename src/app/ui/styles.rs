use ratatui::style::{Modifier, Style};
use ratatui::style::palette::tailwind::{BLUE, GREEN, SLATE};
use ratatui::{style::Color, text::Line, widgets::ListItem};

use crate::app::base::projects::Project;
use crate::app::base::tasks::{ETaskState, SubTask, Task};

pub const TODO_HEADER_STYLE: Style = Style::new()
    .fg(Color::Rgb(145, 174, 190))   // cold desaturated blue
    .bg(Color::Rgb(12, 15, 18));     // graphite black

pub const NORMAL_ROW_BG: Color =
    Color::Rgb(15, 18, 22);

pub const ALT_ROW_BG_COLOR: Color =
    Color::Rgb(18, 22, 27);

pub const SELECTED_STYLE: Style = Style::new()
    .fg(Color::Rgb(218, 228, 234))   // cold white
    .bg(Color::Rgb(38, 49, 58))      // wet asphalt / blue-gray
    .add_modifier(Modifier::BOLD);

pub const TEXT_FG_COLOR: Color =
    Color::Rgb(172, 184, 191);       // overcast gray

pub const COMPLETED_TEXT_FG_COLOR: Color =
    Color::Rgb(104, 126, 137);       // faded cold gray-blue

impl From<&Project> for ListItem<'_> {
    fn from(value: &Project) -> Self {
        ListItem::new(Line::styled(format!("{}", value.title), TEXT_FG_COLOR))
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
