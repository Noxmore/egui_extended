#![doc = include_str!("../readme.md")]

pub mod prelude;
pub mod tabs;
pub mod util;

pub(crate) mod internal;

use egui::{*, style::Widgets};

pub trait UiExtension
{
	//////////////////////////////////////////////////////////////////////////////////
	//// WIDGETS
	//////////////////////////////////////////////////////////////////////////////////
	


	//////////////////////////////////////////////////////////////////////////////////
	//// UTILITIES
	//////////////////////////////////////////////////////////////////////////////////
	
	/// Clones visuals, runs `add_contents`, then returns visuals to previous state
	fn visuals_stack(&mut self, add_contents: impl FnOnce(&mut Ui));
}

impl UiExtension for Ui
{
	fn visuals_stack(&mut self, add_contents: impl FnOnce(&mut Ui)) {
		let visuals = self.visuals().clone();
		add_contents(self);
		*self.visuals_mut() = visuals;
	}
}



pub trait WidgetsExtension {
	fn set_all_rounding(&mut self, rounding: Rounding);
}
impl WidgetsExtension for Widgets {
	fn set_all_rounding(&mut self, rounding: Rounding) {
		self.active.rounding = rounding;
		self.hovered.rounding = rounding;
		self.inactive.rounding = rounding;
		self.noninteractive.rounding = rounding;
		self.open.rounding = rounding;
	}
}


pub trait WidgetExtension: Widget + Sized {
	fn ui_sized(self, ui: &mut Ui, size: impl Into<Vec2>) -> Response {
		ui.add_sized(size, self)
	}
	fn ui_enabled(self, ui: &mut Ui, enabled: bool) -> Response {
		ui.add_enabled(enabled, self)
	}
	fn ui_sized_enabled(self, ui: &mut Ui, size: impl Into<Vec2>, enabled: bool) -> Response {
		let mut response = None;
		ui.add_enabled_ui(enabled, |ui| response = Some(ui.add_sized(size, self)));
		// This is safe because, even though the rust compiler doesn't know it, the code in the above closure will run.
		response.unwrap()
	}
	fn ui_visible(self, ui: &mut Ui, visible: bool) -> Response {
		ui.add_visible(visible, self)
	}
}
impl<T: Widget + Sized> WidgetExtension for T {}