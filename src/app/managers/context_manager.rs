use tracing::info;

use crate::app::{base::{
    projects::{projects_to_widget_list, Project}, tasks::{sub_tasks_to_widget_list, tasks_to_widget_list, Task}, Target, TargetInfo, TargetStatusInfo
}, events::{event_listener::EventsListener, Response}, ui::views::list_view::ListViewItem};

use std::sync::{Arc, Mutex, OnceLock, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct ContextManager {
    projects: Vec<Project>,
    tasks: Vec<Task>,
	active_target: Option<(Target, i64, Option<i64>)>,
	focused_target: Option<(Target, i64, Option<i64>)>
}

static INSTANCE: OnceLock<Arc<RwLock<ContextManager>>> = OnceLock::new();

impl ContextManager {
    fn instance() -> &'static Arc<RwLock<Self>> {
		INSTANCE.get_or_init(|| {
			Arc::new(RwLock::new(Self {
				projects: vec![],
				tasks: vec![],
				active_target: None,
				focused_target: None
			}))
		})
    }

    pub fn get() -> RwLockReadGuard<'static, Self> {
		Self::instance().read().unwrap()
    }

	pub fn get_mut() -> RwLockWriteGuard<'static, Self> {
		Self::instance().write().unwrap()
	}

    pub fn handle_response(&mut self, response: Response) {
		match response {
			Response::Database(database_response) => {
				for project in &mut self.projects {
					project.handle_response(&database_response);
				}
			}
			_=> {
				
			}
		}
	}

	pub fn go_up(&mut self) {
		match self.active_target.clone() {
			Some((Target::Task, _, Some(project_id))) => {
				self.active_target = Some((Target::Project, project_id, None));
			}
			Some((Target::Task, _, None)) => {
				self.active_target = Some((Target::All, -1, None));
			}
			Some((Target::SubTask, id, project_id)) => {
				self.active_target = Some((Target::Task, id, project_id));
			}
			Some((Target::Project, _, None)) => {
				self.active_target = Some((Target::All, -1, None));
			}
			_=> {
				// self.active_target = None;
			}
		}
	}

	pub fn init(&mut self, projects: Vec<Project>, tasks: Vec<Task>) {
		self.projects = projects;
		self.tasks = tasks;
	}

	pub fn set_active_target(&mut self, new_target: Option<(Target, i64, Option<i64>)>) { 
		self.active_target = new_target;
	}

	pub fn set_focused_target(&mut self, new_target: Option<(Target, i64, Option<i64>)>) { 
		self.focused_target = new_target;
	}

	pub fn get_focused_target_status_info(&self) -> Option<TargetStatusInfo> {
		match &self.focused_target {
			Some((Target::Project, id, _)) => {
				None
				// Some(TargetStatusInfo::Project(self.))
			}
			_=>{
				None
			}
		}
	}

	pub fn get_active_taret_list_view(&self) -> Vec<Box<dyn ListViewItem>> {
		match &self.active_target {
			Some((target,_, _)) => {
				self.get_target_list_view(target.clone())
			}
			_=> {
				vec![]
			}
		}
	}

	pub fn get_target_list_view(&self, target: Target) -> Vec<Box<dyn ListViewItem>> {
		let mut view_data: Vec<Box<dyn ListViewItem>> = vec![];
		match target {
			Target::Task => {
				match self.active_target {
					Some((Target::Task, _, Some(project_id))) => {
						for project in &self.projects {
							if project.id == project_id {
								view_data = tasks_to_widget_list(&project.tasks);
							}
						}
					}
					_=> {
						view_data = tasks_to_widget_list(&self.tasks);
					}
				}
			},
			Target::SubTask => {
				match self.active_target {
					Some((Target::SubTask, id, Some(project_id))) => {
						for project in &self.projects {
							if project.id == project_id {
								for task in &project.tasks {
									if task.id == id {
										view_data = sub_tasks_to_widget_list(&task.sub_tasks);
										break;
									}
								}
							}
						}
					}
					Some((Target::SubTask, id, None)) => {
						for task in &self.tasks {
							if task.id == id {
								view_data = sub_tasks_to_widget_list(&task.sub_tasks);
								break;
							}
						}
					}
					_=> {
					}
				}

			},
			Target::Project => {
				view_data = projects_to_widget_list(&self.projects);
			},
			Target::All => {
				view_data = projects_to_widget_list(&self.projects);
				view_data.extend(tasks_to_widget_list(&self.tasks));
			},
			_ => {
			},
		}

		view_data
	}
}
