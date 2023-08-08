#![doc = include_str!("../readme.md")]

pub mod prelude;
pub mod connected_tabs;
pub mod widget_placement;
pub mod switch;
pub mod custom_frame;

pub(crate) mod internal;

pub(crate) use internal::*;
pub(crate) use crate::prelude::*;


#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum WidgetState
{
	Active,
	Hovered,
	#[default]
	Inactive,
	Noninteractive,
	Open,
}

pub trait WidgetsExtension
{
	/// Sets rounding for all widget states
	fn set_all_rounding(&mut self, rounding: Rounding);

	/// Modifies all widget state visuals using the specified modifier
	fn modify_all(&mut self, modifier: impl Fn(&mut WidgetVisuals));

	/// Returns the [WidgetVisuals] with the specified state
	fn get_state(&self, state: WidgetState) -> &WidgetVisuals;
	/// Returns the [WidgetVisuals] with the specified state
	fn get_state_mut(&mut self, state: WidgetState) -> &mut WidgetVisuals;
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

	fn get_state(&self, state: WidgetState) -> &WidgetVisuals {
		match state {
			WidgetState::Active => &self.active,
			WidgetState::Hovered => &self.hovered,
			WidgetState::Inactive => &self.inactive,
			WidgetState::Noninteractive => &self.noninteractive,
			WidgetState::Open => &self.open,
		}
	}

	fn get_state_mut(&mut self, state: WidgetState) -> &mut WidgetVisuals {
		match state {
			WidgetState::Active => &mut self.active,
			WidgetState::Hovered => &mut self.hovered,
			WidgetState::Inactive => &mut self.inactive,
			WidgetState::Noninteractive => &mut self.noninteractive,
			WidgetState::Open => &mut self.open,
		}
	}
}


pub trait ResponseExtension
{
	fn get_widget_state(&self) -> WidgetState;
}
impl ResponseExtension for Response
{
	fn get_widget_state(&self) -> WidgetState {
		if !self.sense.interactive() {WidgetState::Noninteractive}
		else if self.is_pointer_button_down_on() || self.has_focus() {WidgetState::Active}
		else if self.hovered() || self.highlighted() {WidgetState::Hovered}
		else {WidgetState::Inactive}
	}
}


pub trait ColorExtension
{
	fn grayed(self, fraction: f32) -> Self;
	fn lighten(self, fraction: f32) -> Self;
	fn darken(self, fraction: f32) -> Self;
}
impl ColorExtension for Color32
{
	fn grayed(self, fraction: f32) -> Self {
		Rgba::from(self).grayed(fraction).into()
	}

	fn lighten(self, fraction: f32) -> Self {
		Rgba::from(self).lighten(fraction).into()
	}

	fn darken(self, fraction: f32) -> Self {
		Rgba::from(self).darken(fraction).into()
	}
}
impl ColorExtension for Rgba
{
	fn grayed(self, fraction: f32) -> Self {
		lerp(self..=Rgba::from_gray(0.5), fraction)
	}

	fn lighten(self, fraction: f32) -> Self {
		lerp(self..=Rgba::WHITE, fraction)
	}

	fn darken(self, fraction: f32) -> Self {
		lerp(self..=Rgba::BLACK, fraction)
	}
}