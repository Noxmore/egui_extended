#![doc = include_str!("../readme.md")]

pub mod prelude;
pub mod tabs;
pub mod widget_placement;

pub(crate) mod internal;
pub(crate) use internal::*;

use egui::{*, style::Widgets};


pub trait WidgetsExtension {
	/// Sets rounding for all widget states
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