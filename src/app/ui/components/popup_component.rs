use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{base::TargetInfo, events::{ButtonRequest, DatabaseRequest, DatabaseResponse, HistoryRequest, PopupResponse, Request, Response}, ui::{components::base::component::Component, sections::ESectionId, views::{base::view::{View, ViewStateController}, popup_view::{PopupData, PopupView}}}};

pub struct PopupComponent {
    view: PopupView,
    active: bool,
	meta: Option<TargetInfo>
}

impl PopupComponent {
    pub fn is_active(&self) -> bool {
        self.active
    }
}

impl Component for PopupComponent {
    fn new() -> Self {
        PopupComponent {
            view: PopupView::default(),
            active: false,
			meta: None
        }
    }

    fn handle_key(&mut self, key: KeyEvent) -> Result<Request, String> {
        if self.view.has_focused_property() {
			let view_state = &mut self.view as &mut dyn ViewStateController;
			view_state.handle_key(key);
            return Ok(Request::Nothing)
        }

        match key.code {
            KeyCode::Char('q') => {
                self.active = false;
            },
            _ => {
				let view_state = &mut self.view as &mut dyn ViewStateController;
                match view_state.handle_key(key) {
					Ok(Request::Button(button_request)) => {
						match button_request {
							Some(request) => {
								match request {
									ButtonRequest::Confirm => {
										let properties = self.view.get_properties();
										self.active = false;
										match &self.meta {
											Some(target) => {
												match target {
													TargetInfo::Project(id, _, _) => {
														let mut db_request = target.clone();
														if id.is_some() {
															// TODO: Update
														} else {
															let mut name = String::default();
															let mut description = String::default();
															for property in properties {
																if property.label == "Name:" {
																	name = property.value;
																} else if property.label == "Description:" {
																	description = property.value;
																}
															}

															return Ok(Request::Database(DatabaseRequest::Add(TargetInfo::Project(None, name, description))));
														}
													},
													TargetInfo::Task(id, name, descr, project_id, proj_name) => {
														if id.is_some() {
															// TODO: Update
														} else {
															let mut name = String::default();
															let mut description = String::default();
															for property in properties {
																if property.label == "Name:" {
																	name = property.value;
																} else if property.label == "Description:" {
																	description = property.value;
																}
															}

															return Ok(Request::Database(DatabaseRequest::Add(TargetInfo::Task(None, name, description, project_id.clone(), proj_name.clone()))));
														}

													},
													TargetInfo::SubTask(id, _, _, parnet_task, _, _, _) => {
														if id.is_some() {
															// TODO: Update
														} else {
															let mut name = String::default();
															let mut description = String::default();
															for property in properties {
																if property.label == "Name:" {
																	name = property.value;
																} else if property.label == "Description:" {
																	description = property.value;
																}
															}

															return Ok(Request::Database(DatabaseRequest::Add(TargetInfo::SubTask(None, name, description, *parnet_task, String::default(), None, None))));
														}
													},
													_ => {}
												}
											}
											_=> {}
										}
									}
									ButtonRequest::Cancel => {
										self.active = false;
									}
									_=>{}
								}
							}
							_ => {
								
							}
						}
					}
					_ => {}
				}
            },
        }

        Ok(Request::Nothing)
    }

    fn handle_response(&mut self, response: Response) {
        match response {
            Response::Popup(popup_response) => {
                match popup_response {
                    PopupResponse::Show(data) => {
						self.meta = data.meta.clone();
                        self.view.set_data(&data);
                        self.active = true;
                    },
                    _ => {},
                }
            },
            _ => {
            }
        }
    }

    fn render_frame(&mut self, frame: &mut ratatui::prelude::Frame, area: ratatui::prelude::Rect) {
        self.view.render_frame(frame, area);
    }
}
