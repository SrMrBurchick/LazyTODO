use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph, Widget, Wrap},
};

use crate::app::{base::Target, ui::{components::user_input::EInputMode, views::base::view::ViewStateController}};
use crate::app::base::tasks::ETaskState;
use crate::app::events::Request;
use crate::app::ui::styles;
use crate::app::ui::views::base::view::View;
use crate::app::ui::views::metric_bar_view::{
    MetricBarData,
    MetricBarView,
};
use crate::app::ui::views::metric_view::{
    MetricData,
    MetricView,
};

use super::{base::view_state::ViewState, property_view::{PropertyData, PropertyView}};

#[derive(Default, Clone)]
pub struct InspectorData {
    pub path : Vec<String>,
    pub item_name : String,
    pub item_description : String,
    pub item_status : Option<ETaskState>,
    pub metrics: Vec<MetricData>,
    pub progress: Option<MetricBarData>,
    pub identity: Option<(Target, i64)>
}

pub struct InspectorView {
    data: Option<InspectorData>,
	state: ViewState,
    selected: bool,
	metrics: Vec<MetricView>,
	progress: Option<MetricBarView>,
	properties: Vec<PropertyView>,
	view_controls: Vec<Box<dyn ViewStateController>>
}

impl InspectorView {
    pub fn new() -> Self {
        InspectorView {
            data: None,
			state: ViewState::default(),
            selected: false,
			metrics: vec![],
			properties: vec![],
			progress: None,
			view_controls: vec![]
        }
    }

    pub fn clean(&mut self) {
        self.data = None;
    }

    pub fn set_data(&mut self, data: &InspectorData) {
        self.data = Some(data.clone());

		// Init child views

		// Properties
		self.properties = vec![
			PropertyView::new(PropertyData {
				label: match data.identity.clone() {
					Some((target, id)) => {
						format!("Name")
					},
					_=> "SPACES".to_string()
				},
				value: data.item_name.clone(),
				read_only: if data.identity.is_some() { false } else { true }
			}),
			PropertyView::new(PropertyData {
				label: match data.identity.clone() {
					Some((_, _)) => {
						format!("Descritpion")
					},
					_=> "SPACES".to_string()
				},
				value: data.item_description.clone(),
				read_only: if data.identity.is_some() { false } else { true }
			}),
		];

				// //
				// // Description
				// //
				// Paragraph::new(data.item_description.as_str())
				// 	.style(Style::default().fg(styles::TITLE_FG_COLOR))
				// 	.wrap(Wrap { trim: true })
				// 	.render(description_area, buf);

				// let status_name = match &data.item_status {
				// 	Some(ETaskState::Todo) => Some("TODO"),
				// 	Some(ETaskState::InProgress) => Some("IN PROGRESS"),
				// 	Some(ETaskState::Completed) => Some("COMPLETED"),
				// 	None => None,
				// };

				// let type_text = match status_name {
				// 	Some(status) => {
				// 		format!("{target_name} · {status}")
				// 	}

				// 	None => {
				// 		target_name.to_string()
				// 	}
				// };

				// Paragraph::new(type_text)
				// 	.style(Style::default().fg(styles::TODO_HEADER_STYLE.fg.unwrap_or(styles::TITLE_FG_COLOR)))
				// 	.render(type_area, buf);

		self.metrics.clear();
		self.progress = None;
        //
        // Metrics
        //
        if !data.metrics.is_empty() {
            for metric in data.metrics.iter()
            {
				self.metrics.push(MetricView::new(metric));
            }
        }

        if let Some(progress) = &data.progress {
			self.progress = Some(MetricBarView::new(progress));
        }
    }

    fn handle_navigation(&mut self, key: crossterm::event::KeyEvent)  -> Result<crate::app::events::Request, String> {
        Ok(Request::Nothing)
    }
}

impl View for InspectorView {
    type ViewData = InspectorData;

    fn render_data(&mut self, area: Rect, buf: &mut Buffer, data: &Self::ViewData) {
		self.set_data(data);
		self.render(area, buf);
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
    }

	fn render_frame(&mut self, frame: &mut ratatui::Frame, area: Rect) {
		let block = Block::new()
			.title(Line::styled(
				" INSPECT ",
				Style::default().fg(styles::TODO_HEADER_STYLE.fg.unwrap_or(styles::TITLE_FG_COLOR)))
			)
			.title_alignment(Alignment::Center)
			.borders(Borders::ALL)
			.border_style(Style::default().fg(if self.selected {styles::SELECTED_SECTION_COLOR} else {styles::METRIC_BORDER_COLOR}))
			.style(Style::default().bg(styles::NORMAL_ROW_BG))
			.padding(Padding::horizontal(1));

		let inner = block.inner(area);

		frame.render_widget(block, area);

        match &self.data {
            Some(data) => {
				let metrics_height = if self.metrics.is_empty() { 0 } else { 4 };
				let progress_height = if self.progress.is_some() { 4 } else { 0 };

				let [
					breadcrumb_area,
					header_area,
					separator_area,
					metrics_area,
					progress_area,
					_remaining,
				] = Layout::vertical([
					Constraint::Length(2),
					Constraint::Min(3),
					Constraint::Length(1),
					Constraint::Length(metrics_height),
					Constraint::Length(progress_height),
					Constraint::Min(0),
				]).areas(inner);

				//
				// Breadcrumb
				//

				let mut breadcrumb_spans = Vec::new();

				for (index, part) in data
					.path
					.iter()
					.chain(std::iter::once(&data.item_name))
					.enumerate()
				{
					if index != 0 {
						breadcrumb_spans.push(Span::styled(
							" / ",
							Style::default().fg(styles::METRIC_BORDER_COLOR)
						));
					}

					breadcrumb_spans.push(Span::styled(
						part.as_str(),
						Style::default().fg(styles::METRIC_LABEL_COLOR),
					));
				}

				frame.render_widget(Paragraph::new(Line::from(breadcrumb_spans))
					.wrap(Wrap { trim: true }), breadcrumb_area);

				if !self.properties.is_empty() {
					let property_areas = Layout::vertical(vec![
						Constraint::Fill(1);
						self.properties.len()
					]).split(header_area);

					for (property_view, property_area) in self.properties.iter_mut().zip(property_areas.iter())
					{
						property_view.render_frame(frame, *property_area);
					}
				}

				//
				// Separator
				//
				frame.render_widget(Block::new()
					.borders(Borders::TOP)
					.border_style(Style::default().fg(styles::METRIC_BORDER_COLOR)), separator_area);

				//
				// Metrics
				//
				if !self.metrics.is_empty() {
					let metric_areas = Layout::horizontal(vec![
						Constraint::Fill(1);
						data.metrics.len()
					]).split(metrics_area);

					for (metric_view, metric_area) in self.metrics.iter_mut().zip(metric_areas.iter())
					{
						metric_view.render(*metric_area, frame.buffer_mut());
					}
				}

				//
				// Progress
				//
				match &mut self.progress {
					Some(progress_view) => {
						progress_view.render(progress_area, frame.buffer_mut());
					}
					_=>{}
				};
			},
            None => {
            },
        }
	}
}

impl ViewStateController for InspectorView{
    fn mark_selected(&mut self, is_selected: bool) {
        self.selected = is_selected;
    }

    fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Result<crate::app::events::Request, String> {
		let input_result: Result<Request, String>;
		match self.state.selected {
			Some(id) => {
				match self.properties.get_mut(id) {
					Some(view) => {
						let view_state = view as &mut dyn ViewStateController;
						input_result = view_state.handle_key(key);
						if view.get_mode() == Some(EInputMode::Editing) {
							return input_result;
						}
					}
					_ => {
						input_result = Ok(Request::Nothing);
					}
				}
			}
			_ => {
				input_result = Ok(Request::Nothing);
			}
		}

        match key.code {
            KeyCode::Char('h') => self.state.select(None),
            KeyCode::Char('j') | KeyCode::Down | KeyCode::Right => self.state.select_next(),
            KeyCode::Char('k') | KeyCode::Up | KeyCode::Left => self.state.select_previous(),
            _ => {
            }
        }
		let views_states: Vec<&mut dyn ViewStateController> = self.properties
			.iter_mut()
			.map(|view| view as &mut dyn ViewStateController)
			// .chain(
			// 	self.metrics
			// 		.iter_mut()
			// 		.map(|view| view as &mut dyn ViewStateController)
			// )
			.collect();

		self.state.handle_secleceted(views_states);

		input_result
    }
}
