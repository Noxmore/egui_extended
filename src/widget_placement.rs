use crate::prelude::*;

pub trait PlaceWidget: Widget + Sized
{
	/// Creates a `WidgetPlacement` that wraps this widget, see [WidgetPlacement] documentation for more details
	fn place(self) -> WidgetPlacement<Self> {
		WidgetPlacement::new(self)
	}
}

impl<T: Widget + Sized> PlaceWidget for T {}


/// Wraps around a widget, and is used to more easily place a widget
/// with custom properties such as size, margin, enabled status, etc
/// 
/// # Examples
/// ```
/// use egui_extended::prelude::*;
/// use egui::*;
/// 
/// egui::__run_test_ui(|ui|
/// {
/// 	Button::new("test")
/// 		// Configure other button properties here
/// 		.place()
/// 			.margin(20.)
/// 			.size([50., 50.])
/// 			.ui(ui);
/// });
/// ```
pub struct WidgetPlacement<T: Widget + Sized>
{
	widget: T,

	margin: Option<Margin>,
	size: Option<Vec2>,
	enabled: Option<bool>,
	visible: Option<bool>,
	visuals_modifiers: Vec<&'static dyn Fn(&mut Visuals)>
}

impl<T: Widget + Sized> WidgetPlacement<T>
{
	fn new(widget: T) -> Self {
		Self {
			widget,
			margin: None,
			size: None,
			enabled: None,
			visible: None,
			visuals_modifiers: Vec::new(),
		}
	}

	crate::builder_set!{/// Sets the margin around the widget
		margin: impl Into<Margin> => Some(margin.into())}

	crate::builder_set!{/// Overrides the size of the widget
		size: impl Into<Vec2> => Some(size.into() )}

	crate::builder_set!{/// Overrides whether the widget is enabled or not
		enabled: bool => Some(enabled)}

	crate::builder_set!{/// Overrides whether the widget is visible or not
		visible: bool => Some(visible)}

	/// Adds a function that modifies the visuals used for this widget
	/// 
	/// Multiple visuals modifiers can be applied to the same widget placement
	/// 
	/// # Examples
	/// ```
	/// use egui_extended::prelude::*;
	/// use egui::*;
	/// 
	/// egui::__run_test_ui(|ui|
	/// {
	/// 	Button::new("Test Button")
	/// 		.place()
	/// 		.size([120., 20.])
	/// 		.visuals_modifier(&visuals_modifiers::no_background)
	/// 		.ui(ui);
	/// });
	/// ```
	pub fn visuals_modifier(mut self, modifier: &'static dyn Fn(&mut Visuals)) -> Self {
		self.visuals_modifiers.push(modifier);
		self
	}
}

impl<T: Widget + Sized> Widget for WidgetPlacement<T>
{
	fn ui(self, ui: &mut Ui) -> Response
	{
		let mut response = None;

		ui.scope(|ui|
		{
			// Apply visuals modifiers
			let visuals = ui.visuals_mut();
			for modifier in self.visuals_modifiers {
				modifier(visuals);
			}

			if let Some(value) = self.enabled { ui.set_enabled(value); }
			if let Some(value) = self.visible { ui.set_visible(value); }

			// Places the widget, defined as a closure to deal with margin handling
			let place_widget = |ui: &mut Ui| {
				// If a custom size is defined, let's use that
				if let Some(size) = self.size {
					response = Some(ui.add_sized(size, self.widget));
				}
				else {
					response = Some(self.widget.ui(ui));
				}
			};

			// If a custom margin is defined, let's add space around this widget
			if let Some(margin) = self.margin {
				ui.add_space(margin.top);
				ui.horizontal(|ui|
				{
					ui.add_space(margin.left);
	
					ui.vertical(place_widget);
	
					ui.add_space(margin.right);
				});
				ui.add_space(margin.bottom);
			}

			// If not, just place it
			else { place_widget(ui); }
		});

		// `response` is set in closures that the rust compiler doesn't know will be run, so some option trickery is in order
		response.unwrap()
	}
}