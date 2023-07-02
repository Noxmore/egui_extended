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