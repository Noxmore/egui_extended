use crate::prelude::*;
use egui::*;

/// Widget for switching between multiple tabs, current tab can be stored in `Ui` memory, or a specified `&mut usize`.
/// 
/// # Examples
/// ```
/// ui.vertical_centered(|ui|
/// {
/// 	Tabs::new("example_tabs", false)
/// 		.tab("Tab #1")
/// 		.tab("Second Tab")
/// 		.rounding_override(Rounding::same(15.))
/// 		.show(ui, |ui, i|
/// 	{
/// 		Separator::default().shrink(100.).ui(ui);
///
/// 		ui.vertical(|ui|
/// 		{
/// 			ui.heading(format!("Tab number: {i}"));
/// 		});
/// 	});
/// });
/// ```
#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct Tabs<'a>
{
	id: Id,

	custom_index_backend: Option<&'a mut usize>,
	tabs: Vec<WidgetText>,
	default: usize,

	rounding_override: Option<Rounding>,
	bg_fill_override: Option<Color32>,
	outer_button_margin: Vec2,
	inner_button_margin: f32,
	bg_margin: f32,
	selected_line: (Stroke, f32),
	selected_line_y_offset: f32,

	vertical: bool,
}

impl<'a> Tabs<'a>
{
	pub fn new(id_source: impl std::hash::Hash, vertical: bool) -> Self {
		Self {
			id: Id::new(id_source),

			custom_index_backend: Default::default(),
			tabs: Default::default(),
			default: Default::default(),

			rounding_override: Default::default(),
			bg_fill_override: Default::default(),
			outer_button_margin: vec2(20., 15.),
			inner_button_margin: 3.,
			bg_margin: 1.5,
			selected_line: (Stroke::NONE, 0.),
			selected_line_y_offset: 0.,

			vertical,
		}
	}


	/// Adds a tab to this instance.
	pub fn tab(mut self, tab: impl Into<WidgetText>) -> Self {
		self.tabs.push(tab.into());
		self
	}
	
	crate::builder_set!{/// Sets the tabs used by this instance.
		tabs: Vec<WidgetText>}
	
	pub fn tabs_from_vec<T>(mut self, vec: &Vec<T>, title_fn: impl Fn(&T) -> WidgetText) -> Self {
		self.tabs = vec.iter().map(title_fn).collect();
		self
	}
	
	crate::builder_set!{/// If this function is not called, the currently selected tab will be stored in tmp ui memory.
		index => custom_index_backend: &'a mut usize => Some(custom_index_backend)}

	crate::builder_set!{/// Sets the default tab, will have no effect if a custom index backend is set.
		default_tab => default: usize}

	crate::builder_set!{rounding_override: Rounding => Some(rounding_override)}
	crate::builder_set!{/// The bg fill is the color of the rectangle behind tab buttons.
		bg_fill_override: Color32 => Some(bg_fill_override)}
	crate::builder_set!{/// The volume of the buttons (default: [20.0, 15.0])
		outer_button_margin: impl Into<Vec2> => outer_button_margin.into()}
	crate::builder_set!{/// The margin in-between buttons (default: 3.0)
		inner_button_margin: f32}
	crate::builder_set!{/// How much bigger or smaller the bg element is compared to the buttons (default: 1.5)
		bg_margin: f32}

	/// Draws a line under the currently selected tab, width is a range from 0-1, 1 being 100% of the tab's width.
	pub fn selected_line(mut self, stroke: Stroke, width: f32) -> Self {
		self.selected_line = (stroke, width);
		self
	}

	crate::builder_set!{selected_line_y_offset: f32}

	// Some internal functions

	fn get_index(&mut self, ui: &Ui) -> usize {
		if let Some(i) = &self.custom_index_backend {**i}
		else {ui.memory_mut(|m| m.data.get_temp(self.id).unwrap_or(self.default))}
	}

	fn set_index(&mut self, ui: &Ui, index: usize) {
		if let Some(i) = &mut self.custom_index_backend {**i = index}
		else {ui.memory_mut(|m| m.data.insert_temp(self.id, index))}
	}

	// TODO: This documentation kinda sucks, make it more clear

	/// Displays this instance, then ui just after it, with the index of this instance supplied.
	pub fn show(mut self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui, usize)) -> Response
	{
		let index = self.get_index(ui);
		let response = self.ui(ui);
		add_contents(ui, index);

		response
	}

	/// Displays this instance the same as `show`, but based on the `Vec` supplied.
	pub fn show_vec<T>(mut self, ui: &mut Ui, vec: &Vec<T>, title_fn: impl Fn(&T) -> WidgetText, add_contents: impl FnOnce(&mut Ui, &T)) -> Response
	{
		self.tabs = vec.iter().map(title_fn).collect();
		self.show(ui, |ui, i| add_contents(ui, &vec[i]))
	}

	/// Mutable version of `show_vec`
	pub fn show_vec_mut<T>(mut self, ui: &mut Ui, vec: &mut Vec<T>, title_fn: impl Fn(&T) -> WidgetText, add_contents: impl FnOnce(&mut Ui, &mut T)) -> Response
	{
		self.tabs = vec.iter().map(title_fn).collect();
		self.show(ui, |ui, i| add_contents(ui, &mut vec[i]))
	}
}

impl Widget for Tabs<'_>
{
	fn ui(mut self, ui: &mut Ui) -> Response
	{
		let rounding = self.rounding_override.unwrap_or(ui.visuals().widgets.noninteractive.rounding);
		let bg_color = self.bg_fill_override.unwrap_or(ui.visuals().widgets.inactive.bg_fill);

		// Find the biggest button, to make button size consistent
		// TODO: Support different sizes per button
		let mut button_size = Vec2::ZERO;
		for tab in &self.tabs {
			button_size = button_size.max(tab.clone().into_galley(ui, None, f32::MAX, TextStyle::Button).size());
		}

		// Apply outer margin
		button_size += self.outer_button_margin;

		// Calculate total size of this widget then allocate it
		let mut tabs_size = button_size;
		if self.vertical { tabs_size.y = (tabs_size.y + self.inner_button_margin) * self.tabs.len() as f32 } 
		else { tabs_size.x = (tabs_size.x + self.inner_button_margin) * self.tabs.len() as f32 }

		let (rect, mut response) = ui.allocate_exact_size(tabs_size, Sense::click());

		// PATCH: button alignment, i think i need to do this because `ui.allocate_exact_size` does allocate the exact size i want
		// ...or my math is wrong. But it's fixed now and that's all that matters
		{
			if !self.vertical {
				button_size.x = (rect.width() - self.inner_button_margin) / self.tabs.len() as f32;
			}
			else {
				button_size.y = (rect.height() - self.inner_button_margin) / self.tabs.len() as f32;
			}
		}
		
		// Paint bg element
		if bg_color.a() > 0 {
			ui.painter().rect_filled(
				rect.expand(self.bg_margin),
				rounding,
				bg_color
			);
		}

		// Buttons
		let mut current_index = self.get_index(ui); // Stores the current index to be changed


		//////////////////////////////////////////////////////////////////////////////////
		//// BUTTONS
		//////////////////////////////////////////////////////////////////////////////////

		for (i, tab) in self.tabs.iter().enumerate()
		{
			ui.visuals_stack(|ui|
			{
				// Rounding override application
				if let Some(rounding) = self.rounding_override {
					ui.visuals_mut().widgets.set_all_rounding(rounding);
				}

				// Calculate button position
				let mut button_pos = rect.min;
				let f32i = i as f32; // To avoid tons of conversions

				if self.vertical { button_pos.y += (button_size.y + self.inner_button_margin) * f32i }
				else { button_pos.x += (button_size.x + self.inner_button_margin) * f32i }


				// Display button
				let button = ui.put(Rect::from_min_size(button_pos, button_size), SelectableLabel::new(i == current_index, tab.clone()));


				// Draw selectable_line under button, if width is above 0
				if self.selected_line.1 > 0. && i == current_index
				{
					let center = button_pos.x + button_size.x / 2.;
					let offset = (button_size.x / 2.) * self.selected_line.1;
					ui.painter().hline((center-offset)..=(center+offset), button_pos.y + button_size.y + self.selected_line_y_offset, self.selected_line.0);
				}
	
				// If clicked, change the current index
				if button.clicked() {
					current_index = i;
				}
				// If mouse is over button, change the outputted response to the button's response 
				if button.hovered() {
					response = button;
				}
			});
		}

		self.set_index(ui, current_index);

		response
	}
}