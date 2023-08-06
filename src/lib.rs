#![doc = include_str!("../readme.md")]

pub mod prelude;
pub mod connected_tabs;
pub mod widget_placement;
pub mod visuals_modifiers;
pub mod switch;
pub mod custom_frame;

pub(crate) mod internal;

pub(crate) use internal::*;
pub(crate) use crate::prelude::*;


pub trait WidgetsExtension
{
	/// Sets rounding for all widget states
	fn set_all_rounding(&mut self, rounding: Rounding);

	/// Modifies all widget state visuals using the specified modifier
	fn modify_all(&mut self, modifier: impl Fn(&mut WidgetVisuals));
}
impl WidgetsExtension for Widgets
{
	fn set_all_rounding(&mut self, rounding: Rounding) {
		self.modify_all(|widget| widget.rounding = rounding);
	}

	fn modify_all(&mut self, modifier: impl Fn(&mut WidgetVisuals)) {
		modifier(&mut self.active);
		modifier(&mut self.hovered);
		modifier(&mut self.inactive);
		modifier(&mut self.noninteractive);
		modifier(&mut self.open);
	}
}