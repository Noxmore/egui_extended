//! Functions that modify visuals
//! 
//! Usually used with widget placements

use crate::prelude::*;

/// Removes the background of widgets
pub fn no_background(visuals: &mut Visuals) {
	visuals.widgets.modify_all(|widget| {
		widget.bg_fill = Color32::TRANSPARENT;
		widget.weak_bg_fill = Color32::TRANSPARENT;
	});
}