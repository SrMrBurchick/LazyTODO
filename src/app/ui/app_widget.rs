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
use ratatui::{DefaultTerminal, Frame, symbols};
use tokio::sync::mpsc;

use crate::app::base::Target;
use crate::app::base::projects::Project;
use crate::app::events::{DatabaseRequest, DatabaseResponse, PopupRequest, Request, Response};
use crate::app::ui::components::base::component::Component;
use crate::app::ui::components::navigation::NavigationComponent;
use crate::app::ui::components::popup_component::PopupComponent;
use crate::app::ui::sections::ESectionId;
use crate::app::ui::sections::base::section::Section;
use crate::app::ui::sections::content_section::ContentSection;
use crate::app::ui::sections::inspector_section::InspectorSection;
use crate::app::ui::sections::spaces_section::SpacesSection;
use crate::app::ui::views::base::view::View;
use crate::app::ui::views::popup_view::{PopupData, PopupView};
use crossterm::event::{Event, EventStream};
use futures::StreamExt;

pub struct AppWidget {
    sections: Vec<Box<dyn Section>>,
    navigation_component: NavigationComponent,
    popup_component: PopupComponent,
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
            navigation_component: NavigationComponent::new(),
            popup_component: PopupComponent::new()
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
            match terminal.draw(|frame| {
                let area= frame.area();
                self.render_frame(area, frame);
            }) {
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
        match response.clone() {
            Response::Popup(popup_response) => {
                self.popup_component.handle_response(response);
            }
            _ => {
                for section in self.sections.iter_mut() {
                    match section.handle_response(&response) {
                        Ok(request) => {
                            match request {
                                Request::Nothing => {

                                },
                                _ => {
                                    self.sender.send(request).await;
                                },
                            }
                        },
                        Err(_) => {},
                    }
                }
            },
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
                                        section.mark_selected(true);
                                        self.navigation_component.set_active_section(Some(section_id.clone()));
                                        match section.handle_key(key) {
                                            Ok(request) => {
                                                self.sender.send(request).await;
                                            },
                                            Err(_) => {},
                                        };
                                    } else {
                                        section.mark_selected(false);
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

    fn render_popup(&mut self, area: Rect, frame: &mut Frame) {
        if self.popup_component.is_ready_to_show() {
            self.popup_component.render_frame(frame, area);
        }
    }

    fn render_frame(&mut self, area: Rect, frame: &mut Frame) {
        let layout = Layout::horizontal([
            Constraint::Percentage(20),
            Constraint::Percentage(50),
            Constraint::Percentage(30),
        ]);

        let areas = layout.split(area);
        for (section, area) in self.sections.iter_mut().zip(areas.iter()) {
            section.render(*area, frame);
        }

        self.render_popup(area, frame);
    }
}
