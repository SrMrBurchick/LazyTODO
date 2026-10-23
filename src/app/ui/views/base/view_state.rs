use crate::app::ui::views::base::view::ViewStateController;

pub struct ViewState {
    pub selected: Option<usize>,
}

impl Default for ViewState {
	fn default() -> Self {
		ViewState {
			selected: None,
		}
	}
}


impl ViewState {
    pub const fn select(&mut self, index: Option<usize>) {
        self.selected = index;
    }

    pub fn select_next(&mut self) {
		let next = self.selected.map_or(0, |i| i.saturating_add(1));
		self.select(Some(next));
    }

    pub fn select_previous(&mut self) {
        let previous = self.selected.map_or(usize::MAX, |i| i.saturating_sub(1));
        self.select(Some(previous));
    }

	pub fn handle_secleceted(&mut self, views: Vec<&mut dyn ViewStateController>) {
		let size = views.len();

		if self.selected.is_some_and(|s| s >= size) {
			self.select(Some(size.saturating_sub(1)));
		}

		for (index, state) in views.into_iter().enumerate() {
			match self.selected {
				Some(id) => {
					state.mark_selected(id == index);
				},
				None => {
					state.mark_selected(false);
				}
			};
		
		}
	}
}
