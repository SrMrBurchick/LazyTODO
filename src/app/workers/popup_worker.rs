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
                match target {
                    TargetInfo::Project(_, _, _) => {
                        let mut data = PopupData::default();
                        data.title = String::from("Create Project");
                        data.properties = vec![
                            PropertyData {
                                label: String::from("New Project:"),
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
