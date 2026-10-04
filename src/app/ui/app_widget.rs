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
use crate::app::events::{DatabaseRequest, Request, Response, DatabaseResponse};
use crate::app::ui::components::base::component::Component;
use crate::app::ui::components::navigation::NavigationComponent;
use crate::app::ui::sections::ESectionId;
use crate::app::ui::sections::base::section::Section;
use crate::app::ui::sections::content_section::ContentSection;
use crate::app::ui::sections::inspector_section::InspectorSection;
use crate::app::ui::sections::spaces_section::SpacesSection;
use crossterm::event::{Event, EventStream};
use futures::StreamExt;

pub struct AppWidget {
    sections: Vec<Box<dyn Section>>,
    navigation_component: NavigationComponent,
    sender: mpsc::Sender<Request>,
    receiver: mpsc::Receiver<Response>,
    should_exit: bool
}

impl AppWidget {
    pub fn new(tx: mpsc::Sender<Request>, rx: mpsc::Receiver<Response>) -> Self {
        AppWidget {
            sections: vec![
                Box::new(SpacesSection::new()),
                Box::new(ContentSection::new()),
                Box::new(InspectorSection::new())
            ],
            sender: tx,
            receiver: rx,
            should_exit: false,
            navigation_component: NavigationComponent::new()
        }
    }

    pub fn initialize(&mut self) {
        for section in self.sections.iter_mut() {
            section.initialize();
        }

        self.navigation_component.set_active_section(Some(ESectionId::Spaces));
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
                        self.handle_response(response).await;
                    }
                }
            }
        }
    }

    async fn handle_response(&mut self, response: Response) {
        for section in self.sections.iter_mut() {
            section.handle_response(&response);
        }
    }

    async fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => {
                self.sender.send(Request::Exit).await;
                self.should_exit = true
            },
            _ => {
                match self.navigation_component.handle_key(key) {
                    Ok(request) => {
                        if (request != Request::Nothing) {
                            self.sender.send(request).await;
                            return;
                        }
                    },
                    Err(_) => {},
                }

                match self.navigation_component.get_active_section() {
                    Some(active_id) => {
                        for section in self.sections.iter_mut() {
                            match section.get_id() {
                                Some(section_id) => {
                                    let is_same_section = std::mem::discriminant(&section_id) == std::mem::discriminant(&active_id);
                                    if is_same_section {
                                        match section.handle_key(key) {
                                            Ok(request) => {
                                                self.sender.send(request).await;
                                            },
                                            Err(_) => {},
                                        };
                                    } else {

                                    }
                                },
                                None => {},
                            }
                        }
                    },
                    None => {},
                }

            }
        }
    }

    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new(format!("TODO: Section name here"))
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
            let title = env!("CARGO_PKG_NAME");
            let version = env!("CARGO_PKG_VERSION");
            Paragraph::new(format!("{title} {version}"))
            .centered()
            .render(area, buf);
    }

    pub fn render_content(&mut self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::horizontal([
            Constraint::Percentage(20),
            Constraint::Percentage(50),
            Constraint::Percentage(30),
        ]);

        let areas = layout.split(area);

        for (section, area) in self.sections.iter_mut().zip(areas.iter()) {
            section.render(*area, buf);
        }
    }
}

impl Widget for &mut AppWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        self.render_content(area, buf);
    }
}
