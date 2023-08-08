use crate::*;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TitleBarButton
{
	Close,
	Maximize,
	Minimize,
}

/// Emulates native title bar buttons, usually used with a custom window frame
/// 
/// # Examples
/// ```ignore
/// // This example uses eframe
/// let title_bar_buttons = TitleBarButtons::new()
/// 	.kind(TitleBarButtonsKind::default())
/// 	.focused(frame.info().window_info.focused)
/// 	.maximized(frame.info().window_info.maximized)
/// 	.show(ui);
/// 
/// for (button, response) in title_bar_buttons {
/// 	if response.clicked() {
/// 		match button {
/// 			TitleBarButton::Close => frame.close(),
/// 			TitleBarButton::Maximize => frame.set_maximized(!frame.info().window_info.maximized),
/// 			TitleBarButton::Minimize => frame.set_minimized(true),
/// 		}
/// 	}
/// }
/// ```
#[derive(Debug, Clone)]
pub struct TitleBarButtons
{
	kind: TitleBarButtonsKind,
	focused: bool,
	maximized: bool,
	icon_size: f32,

	order: Vec<TitleBarButton>,

	rounding_override: Option<Rounding>,
	size_override: Option<Vec2>,
	close_button_color_override: Option<(Color32, Color32)>,
}

impl TitleBarButtons
{
	pub fn new() -> Self {
		Self {
			kind: Default::default(),
			focused: true,
			maximized: false,
			icon_size: 9.,

			#[cfg(target_os = "macos")] order: vec![TitleBarButton::Close, TitleBarButton::Minimize, TitleBarButton::Maximize],
			#[cfg(not(target_os = "macos"))] order: vec![TitleBarButton::Minimize, TitleBarButton::Maximize, TitleBarButton::Close],

			rounding_override: None,
			size_override: None,
			close_button_color_override: None,
		}
	}


	pub fn close_button_color_override(mut self, hovered: Color32, pressed: Color32) -> Self {
		self.close_button_color_override = Some((hovered, pressed));
		self
	}
	


	// Builder functions

	crate::builder_set!{/// The kind of title bar buttons to use (default: Platform dependent)
		kind: TitleBarButtonsKind}

	crate::builder_set!{/// If the window is currently focused (default: true)
		focused: bool}
	crate::builder_set!{/// If the window is currently maximized (default: false)
		maximized: bool}
	crate::builder_set!{/// How big the icons are rendered (default: 9.0)
		icon_size: f32}

	crate::builder_set!{/// The order of the buttons (default: Platform dependent)
		order: Vec<TitleBarButton>}


	crate::builder_set!{/// Overrides the rounding of the buttons, this does not effect circle buttons
		rounding_override: impl Into<Rounding> => Some(rounding_override.into())}

	crate::builder_set!{/// Overrides the size of the buttons, this doesn't effect icons, just the button area
		size_override: impl Into<Vec2> => Some(size_override.into())}




	pub fn show(self, ui: &mut Ui) -> Vec<(TitleBarButton, Response)>
	{
		let button_count = self.order.len();
		let button_size = self.size_override.unwrap_or(self.kind.button_size());
		let (rect, _response) = ui.allocate_at_least(button_size * vec2(button_count as f32, 1.), Sense::click());


		let mut button_responses = Vec::new();

		
		for (i, button) in self.order.into_iter().enumerate()
		{
			// Calculate the size of the individual button
			let button_rect = {
				let mut rect = rect;
				rect.set_width(rect.width() / button_count as f32);
				rect.translate(vec2(rect.width() * i as f32, 0.)).shrink(self.kind.shrink_amount())
			};
			// Then interact with it
			let button_response = ui.interact(button_rect, Id::new(format!("title_bar_button {i}")), Sense::click());

			// Get variables setup for painting
			let (painter, pos, size, visuals, focused, state) =
				(ui.painter(), button_rect.center(), self.icon_size / ui.ctx().pixels_per_point(), ui.visuals(), self.focused, button_response.get_widget_state());

			// Windows buttons backgrounds
			if self.kind == TitleBarButtonsKind::Windows && state != WidgetState::Inactive && state != WidgetState::Noninteractive
			{
				let (close_hovered, close_pressed) = if let Some(c) = self.close_button_color_override {(c.0, c.1)}
					else {(Color32::from_rgb(255, 50, 50), Color32::LIGHT_RED)};

				painter.rect_filled(button_rect, self.rounding_override.unwrap_or(Rounding::none()),
					match button {
						TitleBarButton::Close => if state == WidgetState::Active {close_pressed} else {close_hovered},
						_ => visuals.widgets.get_state(state).bg_fill,
						// _ => if state == WidgetState::Active {Color32::from_gray(50)} else {Color32::DARK_GRAY},
					}
				);
			}

			// Paint it
			match button {
				TitleBarButton::Close => self.kind.paint_close_button(painter, pos, size, visuals, focused, state, self.close_button_color_override),
				TitleBarButton::Maximize => {
					if self.maximized {
						self.kind.paint_restore_button(painter, pos, size, visuals, focused, state);
					} else {
						self.kind.paint_maximize_button(painter, pos, size, visuals, focused, state);
					}
				},
				TitleBarButton::Minimize => self.kind.paint_minimize_button(painter, pos, size, visuals, focused, state),
			}

			button_responses.push((button, button_response));
		}

		button_responses
	}
}

impl Default for TitleBarButtons {
	fn default() -> Self {
		Self::new()
	}
}