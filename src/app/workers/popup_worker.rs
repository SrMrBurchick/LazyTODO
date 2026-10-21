use crate::app::{
    base::{Target, TargetInfo, TargetStatusInfo}, events::{PopupRequest, PopupResponse, Response}, ui::views::{popup_view::PopupData, property_view::{PropertyData, PropertyView}}, workers::base::worker::Worker,
};
use tracing::{error, info, warn};

pub struct PopupWorker {
}

impl PopupWorker {
    pub fn new() -> Self {
        PopupWorker {
        }
    }
}

impl Worker for PopupWorker {
    type WorkerRequest = PopupRequest;

    fn handle_request(&mut self, request: Self::WorkerRequest) -> Result<Response, String> {
        match request {
            PopupRequest::Create(target) => {
				let mut data = PopupData::default();
				data.meta = Some(target.clone());

                match target {
                    TargetInfo::Project(_, _, _) => {
                        data.title = String::from("Create Project");
                        data.properties = vec![
                            PropertyData {
                                label: String::from("Name:"),
                                value: String::default(),
                                read_only: false
                            },
                            PropertyData {
                                label: String::from("Description:"),
                                value: String::default(),
                                read_only: false
                            },
                        ];

                        return Ok(Response::Popup(PopupResponse::Show(data)));
                    },
					TargetInfo::Task(_, _, _, _, project_name) => {
						data.title = String::from("Create Task") + match project_name {
							Some(name) => {String::from(format!(" for project {}", name.clone())) }
							_ => {String::from("")}
						}.as_str();
						data.properties = vec![
							PropertyData {
								label: String::from("Name:"),
								value: String::default(),
								read_only: false
							},
							PropertyData {
								label: String::from("Description:"),
								value: String::default(),
								read_only: false
							},
						];

                        return Ok(Response::Popup(PopupResponse::Show(data)));
					
					},
					TargetInfo::SubTask(_, _, _, _, task_name, _, _) => {
						data.title = String::from("Create Subtask for ") + task_name.as_str();
						data.properties = vec![
							PropertyData {
								label: String::from("Name:"),
								value: String::default(),
								read_only: false
							},
							PropertyData {
								label: String::from("Description:"),
								value: String::default(),
								read_only: false
							},
						];

                        return Ok(Response::Popup(PopupResponse::Show(data)));
					
					},
                    _ => {},
                }
            },
            _ => {

            }
        }
        Ok(Response::Nothing)
    }
}
