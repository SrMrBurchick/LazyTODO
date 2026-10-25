use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::ListItem,
};
use tracing::info;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

use crate::app::base::projects::Project;
use crate::app::base::tasks::{ETaskState, SubTask, Task};

//
// BASE PALETTE
//

// Background hierarchy
pub const BG_ROOT: Color =
    Color::Rgb(8, 12, 16);

pub const BG_PANEL: Color =
    Color::Rgb(12, 18, 24);

pub const BG_PANEL_ALT: Color =
    Color::Rgb(15, 22, 29);

pub const BG_SURFACE: Color =
    Color::Rgb(17, 25, 33);

pub const BG_SELECTED: Color =
    Color::Rgb(28, 43, 53);

// Borders
pub const BORDER_DIM: Color =
    Color::Rgb(30, 47, 58);

pub const BORDER_NORMAL: Color =
    Color::Rgb(40, 61, 73);

pub const BORDER_ACTIVE: Color =
    Color::Rgb(78, 117, 137);

// Text
pub const TEXT_PRIMARY: Color =
    Color::Rgb(202, 214, 221);

pub const TEXT_SECONDARY: Color =
    Color::Rgb(142, 160, 171);

pub const TEXT_MUTED: Color =
    Color::Rgb(100, 117, 128);

pub const TEXT_DISABLED: Color =
    Color::Rgb(74, 88, 97);

// Main cyberpunk accent
pub const ACCENT_PRIMARY: Color =
    Color::Rgb(112, 164, 186);

pub const ACCENT_BRIGHT: Color =
    Color::Rgb(145, 194, 214);

// Semantic colors
pub const STATUS_TODO: Color =
    Color::Rgb(104, 126, 137);

pub const STATUS_ACTIVE: Color =
    Color::Rgb(201, 179, 122);

pub const STATUS_COMPLETED: Color =
    Color::Rgb(118, 170, 148);

pub const STATUS_ERROR: Color =
    Color::Rgb(190, 108, 112);


//
// MAIN UI
//

pub const APP_BG: Color = BG_ROOT;

pub const MAIN_MUTED_COLOR: Color = TEXT_MUTED;

pub const TODO_HEADER_STYLE: Style = Style::new()
    .fg(ACCENT_PRIMARY)
    .bg(BG_PANEL);

pub const SELECTED_STYLE: Style = Style::new()
    .fg(TEXT_PRIMARY)
    .bg(BG_SELECTED)
    .add_modifier(Modifier::BOLD);

pub const SELECTED_SECTION_COLOR: Color =
    BORDER_ACTIVE;


//
// ROWS
//

pub const NORMAL_ROW_BG: Color =
    BG_PANEL;

pub const ALT_ROW_BG_COLOR: Color =
    BG_PANEL_ALT;


//
// TEXT HIERARCHY
//

pub const TITLE_FG_COLOR: Color =
    TEXT_PRIMARY;

pub const DESCRIPTION_FG_COLOR: Color =
    TEXT_MUTED;

pub const COMPLETED_TEXT_FG_COLOR: Color =
    TEXT_SECONDARY;


//
// STATUS
//

pub const TODO_MARK_FG_COLOR: Color =
    STATUS_TODO;

pub const COMPLETED_MARK_FG_COLOR: Color =
    STATUS_COMPLETED;

pub const IN_PROGRESS_MARK_FG_COLOR: Color =
    STATUS_ACTIVE;


//
// INSPECTOR / METRICS
//

pub const METRIC_CARD_BG: Color =
    BG_SURFACE;

pub const METRIC_BORDER_COLOR: Color =
    BORDER_DIM;

pub const METRIC_LABEL_COLOR: Color =
    TEXT_MUTED;

pub const METRIC_VALUE_COLOR: Color =
    TEXT_PRIMARY;

pub const METRIC_HIGHLIGHT_COLOR: Color =
    ACCENT_BRIGHT;

pub const PROGRESS_TRACK_BG: Color =
    BG_PANEL_ALT;


//
// PROPERTY / INPUT
//

pub const PROPERTY_BG: Color =
    BG_SURFACE;

pub const PROPERTY_BORDER_COLOR: Color =
    BORDER_DIM;

pub const PROPERTY_SELECTED_BORDER_COLOR: Color =
    BORDER_ACTIVE;

pub const PROPERTY_EDITING_BORDER_COLOR: Color =
    STATUS_ACTIVE;

pub const PROPERTY_LABEL_COLOR: Color =
    ACCENT_PRIMARY;

pub const PROPERTY_VALUE_COLOR: Color =
    TEXT_PRIMARY;

pub const PROPERTY_READ_ONLY_COLOR: Color =
    TEXT_DISABLED;


//
// POPUP
//

pub const POPUP_BG: Color =
    BG_PANEL;

pub const POPUP_BORDER_COLOR: Color =
    BORDER_ACTIVE;

pub const POPUP_TITLE_COLOR: Color =
    ACCENT_PRIMARY;

pub const POPUP_TEXT_COLOR: Color =
    TEXT_PRIMARY;

pub const POPUP_SELECTED_BG: Color =
    BG_SELECTED;

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
                Style::default().fg(MAIN_MUTED_COLOR),
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
                Style::default().fg(MAIN_MUTED_COLOR),
            ),
        ])
    }
}

pub fn wrap_lines(data: &str, max_width: usize) -> Vec<String> {
    if max_width == 0 {
        return vec![String::new()];
    }

    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0;

    for ch in data.chars() {
        if ch == '\n' {
            lines.push(current_line);
            current_line = String::new();
            current_width = 0;
            continue;
        }

        let char_width = unicode_width::UnicodeWidthChar::width(ch)
            .unwrap_or(0);

        if current_width + char_width > max_width {
            lines.push(current_line);
            current_line = String::new();
            current_width = 0;
        }

        current_line.push(ch);
        current_width += char_width;
    }

    lines.push(current_line);

    lines
}

pub fn get_ui_width(data: &str) -> usize {
	UnicodeWidthStr::width(data)
}
