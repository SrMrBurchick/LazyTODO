use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::palette::tailwind::{BLUE, GREEN, SLATE};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{
    Block, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph,
    StatefulWidget, Widget, Wrap,
};
use ratatui::{DefaultTerminal, symbols};
use tokio::sync::mpsc;

use crate::app::base::Target;
use crate::app::base::projects::Project;
use crate::app::events::{DatabaseRequest, Request, Response};
use crate::app::ui::base::widget_list::{WidgetList, WidgetListItem};
use crate::app::ui::styles;
use crossterm::event::{Event, EventStream};
use futures::StreamExt;

pub struct AppWidget {
    pub list: WidgetList,
    sender: mpsc::Sender<Request>,
    receiver: mpsc::Receiver<Response>,
    should_exit: bool
}

const fn alternate_colors(i: usize) -> Color {
    if i.is_multiple_of(2) {
        styles::NORMAL_ROW_BG
    } else {
        styles::ALT_ROW_BG_COLOR
    }
}

impl AppWidget {
    pub fn new(tx: mpsc::Sender<Request>, rx: mpsc::Receiver<Response>) -> Self {
        AppWidget {
            list: WidgetList::default(),
            sender: tx,
            receiver: rx,
            should_exit: false
        }
    }

    pub async fn run(mut self, terminal: &mut DefaultTerminal) {
        let mut input = EventStream::new();

        while !self.should_exit {
            match terminal.draw(|frame| frame.render_widget(&mut self, frame.area())) {
                Ok(_) => {},
                Err(_) => {
                    return;
                },
            };
            tokio::select! {
                input_event = input.next() => {
                    if let Some(Ok(Event::Key(key))) = input_event {
                        if key.is_press() {
                            self.handle_key(key).await;
                        }
                    }
                }

                response = self.receiver.recv() => {
                    if let Some(response) = response {
                        self.handle_response(response);
                    }
                }
            }
        }
    }

    fn handle_response(&mut self, response: Response) {
        match response {
            Response::Database(database_response) => {
                match database_response {
                    crate::app::events::DatabaseResponse::Projects(projects) => {
                        self.list.items = projects[0].tasks.iter().cloned().map(|task| Box::new(task) as Box<dyn WidgetListItem>).collect();
                    }
                    _ => {},
                }
            }
            _ => {

            }
        }
    }

    async fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.sender.send(Request::Exit).await;
                self.should_exit = true
            },
            KeyCode::Char('h') | KeyCode::Left => self.select_none(),
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
            KeyCode::Char('g') | KeyCode::Home => self.select_first(),
            KeyCode::Char('G') | KeyCode::End => self.select_last(),
            KeyCode::Char('F') => {
                self.sender.send(Request::Database(DatabaseRequest::Get(Target::Project, None))).await;
            },
            _ => {}
        }
    }

    const fn select_none(&mut self) {
        self.list.state.select(None);
    }

    fn select_next(&mut self) {
        self.list.state.select_next();
    }
    fn select_previous(&mut self) {
        self.list.state.select_previous();
    }

    const fn select_first(&mut self) {
        self.list.state.select_first();
    }

    const fn select_last(&mut self) {
        self.list.state.select_last();
    }

    fn render_header(area: Rect, buf: &mut Buffer) {
        let title = env!("CARGO_PKG_NAME");
        let version = env!("CARGO_PKG_VERSION");
        Paragraph::new(format!("{title} {version}"))
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use ↓↑ to move, ← to unselect, → to change status, g/G to go top/bottom.")
            .centered()
            .render(area, buf);
    }


    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("TODO List").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(styles::TODO_HEADER_STYLE)
            .bg(styles::NORMAL_ROW_BG);

        // Iterate through all elements in the `items` and stylize them.
        let items: Vec<ListItem> = self
            .list
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                item.render().bg(alternate_colors(i))
            })
            .collect();

        // // Create a List from all list items and highlight the currently selected one
        let list = List::new(items)
            .block(block)
            .highlight_style(styles::SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
        // same method name `render`.
        StatefulWidget::render(list, area, buf, &mut self.list.state);
    }
}

impl Widget for &mut AppWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let main_layout = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ]);
        let [header_area, content_area, footer_area] = area.layout(&main_layout);

        let content_layout = Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]);
        let [list_area, item_area] = content_area.layout(&content_layout);

        AppWidget::render_header(header_area, buf);
        AppWidget::render_footer(footer_area, buf);
        self.render_list(list_area, buf);
    }
}
