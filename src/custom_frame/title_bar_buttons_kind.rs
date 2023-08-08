use crate::*;

/// The appearance of [TitleBarButtons]
/// 
/// use `TitleBarButtonsKind::default()` to get platform-specific buttons
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TitleBarButtonsKind
{
	Windows,
	Mac,
	/// Linux using the breeze theme
	Breeze,
}

impl Default for TitleBarButtonsKind {
	fn default() -> Self {
		#[cfg(target_os = "linux")] return Self::Breeze;
		#[cfg(target_os = "macos")] return Self::Mac;
		// If no other options are found, we'll just use windows
		#[allow(unreachable_code)] Self::Windows
	}
}

impl TitleBarButtonsKind
{
	/// The size of each button
	pub fn button_size(self) -> Vec2 {
		match self {
			Self::Windows => vec2(40., 20.),
			Self::Mac | Self::Breeze => vec2(20., 20.),
		}
	}

	/// How much the buttons should shrink in place, making the area to click smaller
	pub fn shrink_amount(self) -> f32 {
		match self {
			Self::Breeze => 5.,
			_ => 0.,
		}
	}


	//////////////////////////////////////////////////////////////////////////////////
	//// PAINTERS
	//////////////////////////////////////////////////////////////////////////////////
	
	const ICON_MARGIN: f32 = 0.5;

	pub const MAC_UNFOCUSED_FILL_COLOR: Color32 = Color32::from_gray(172);
	pub const MAC_UNFOCUSED_OUTLINE_COLOR: Color32 = Color32::from_gray(77);

	pub const MAC_CLOSE_FILL_COLOR: Color32 = Color32::from_rgb(255, 95, 86);
	pub const MAC_CLOSE_OUTLINE_COLOR: Color32 = Color32::from_rgb(148, 0, 0);

	pub const MAC_MAXIMIZE_FILL_COLOR: Color32 = Color32::from_rgb(40, 200, 70);
	pub const MAC_MAXIMIZE_OUTLINE_COLOR: Color32 = Color32::from_rgb(10, 100, 10);
	
	pub const MAC_MINIMIZE_FILL_COLOR: Color32 = Color32::from_rgb(255, 190, 50);
	pub const MAC_MINIMIZE_OUTLINE_COLOR: Color32 = Color32::from_rgb(150, 85, 25);


	// Common actions are put in their own functions to avoid code duplication

	fn breeze_stroke(visuals: &Visuals, state: WidgetState) -> Stroke {
		Stroke::new(0.7, if !visuals.dark_mode || state != WidgetState::Inactive {Color32::BLACK} else {Color32::WHITE})
	}

	fn breeze_background(painter: &Painter, pos: Pos2, size: f32, state: WidgetState) {
		match state {
			WidgetState::Hovered => painter.circle_filled(pos, size, Color32::WHITE),
			WidgetState::Active => painter.circle_filled(pos, size, Color32::DARK_GRAY),
			_ => {}
		}
	}

	fn windows_stroke(visuals: &Visuals, focused: bool) -> Stroke {
		let color = if visuals.dark_mode {Color32::WHITE} else {Color32::BLACK};
		Stroke::new(1., if focused {color} else {color.grayed(0.5)})
	}

	fn paint_breeze_arrow(painter: &Painter, pos: Pos2, offset: Vec2, visuals: &Visuals, state: WidgetState) {
		painter.line_segment([pos - vec2(0., offset.y * 0.5), pos + vec2(offset.x, offset.y * 0.5)], Self::breeze_stroke(visuals, state));
		painter.line_segment([pos - vec2(0., offset.y * 0.5), pos + vec2(-offset.x, offset.y * 0.5)], Self::breeze_stroke(visuals, state));
	}


	//////////////////////////////////////////////////////////////////////////////////
	//// CLOSE BUTTON
	//////////////////////////////////////////////////////////////////////////////////
	
	pub fn paint_close_button(self, painter: &Painter, pos: Pos2, size: f32, visuals: &Visuals, focused: bool, state: WidgetState, color_override: Option<(Color32, Color32)>)
	{
		/// The X is the same across all close buttons
		fn paint_x(painter: &Painter, pos: Pos2, size: f32, stroke: Stroke) {
			let offset = vec2(size, size) * TitleBarButtonsKind::ICON_MARGIN;

			// TODO: The lengths of the lines can be slightly off sometimes, probably a rounding error
			painter.line_segment([pos - offset, pos + offset], stroke);
			painter.line_segment([pos - offset * vec2(-1., 1.), pos + offset * vec2(-1., 1.)], stroke);
		}

		match self
		{
			Self::Windows => {
				paint_x(painter, pos, size, Self::windows_stroke(visuals, focused));
			}
			Self::Breeze => {
				// Custom `breeze_background` because the x background is red instead of white
				let (hovered, pressed) = if let Some(c) = color_override {(c.0, c.1)}
					else {(Color32::from_rgb(255, 150, 150), Color32::from_rgb(75, 20, 20))};

				match state {
					WidgetState::Hovered => painter.circle_filled(pos, size, hovered),
					WidgetState::Active => painter.circle_filled(pos, size, pressed),
					_ => {}
				}

				paint_x(painter, pos, size, Self::breeze_stroke(visuals, state));
			}
			Self::Mac => {
				let fill_color = if focused {Self::MAC_CLOSE_FILL_COLOR} else {Self::MAC_UNFOCUSED_FILL_COLOR};
				let outline_color = if focused {Self::MAC_CLOSE_OUTLINE_COLOR} else {Self::MAC_UNFOCUSED_OUTLINE_COLOR};

				painter.circle(pos, size, fill_color, Stroke::new(0.5, outline_color));

				if state != WidgetState::Inactive && state != WidgetState::Noninteractive {
					paint_x(painter, pos, size * 0.8, Stroke::new(1.5, outline_color));
				}
			}
		}
	}


	//////////////////////////////////////////////////////////////////////////////////
	//// MAXIMIZE BUTTON
	//////////////////////////////////////////////////////////////////////////////////
	
	pub fn paint_maximize_button(self, painter: &Painter, pos: Pos2, size: f32, visuals: &Visuals, focused: bool, state: WidgetState)
	{
		let offset = vec2(size, size) * Self::ICON_MARGIN;

		match self
		{
			Self::Windows => {
				painter.rect_stroke(Rect::from_min_max(pos - offset, pos + offset), Rounding::none(), Self::windows_stroke(visuals, focused));
			}
			Self::Breeze => {
				Self::breeze_background(painter, pos, size, state);

				Self::paint_breeze_arrow(painter, pos, offset * 1.15, visuals, state);
			}
			Self::Mac => {
				let fill_color = if focused {Self::MAC_MAXIMIZE_FILL_COLOR} else {Self::MAC_UNFOCUSED_FILL_COLOR};
				let outline_color = if focused {Self::MAC_MAXIMIZE_OUTLINE_COLOR} else {Self::MAC_UNFOCUSED_OUTLINE_COLOR};

				painter.circle(pos, size, fill_color, Stroke::new(0.5, outline_color));

				if state != WidgetState::Inactive && state != WidgetState::Noninteractive {
					let mut mesh = Mesh::default();

					// Bottom left triangle
					mesh.colored_vertex(pos - vec2(offset.x, offset.y * 0.5), outline_color);
					mesh.colored_vertex(pos - vec2(offset.x, -offset.y), outline_color);
					mesh.colored_vertex(pos + vec2(offset.x * 0.5, offset.y), outline_color);

					// Top right triangle
					mesh.colored_vertex(pos + vec2(offset.x, offset.y * 0.5), outline_color);
					mesh.colored_vertex(pos + vec2(offset.x, -offset.y), outline_color);
					mesh.colored_vertex(pos - vec2(offset.x * 0.5, offset.y), outline_color);

					for i in 0..6 {
						mesh.indices.push(i);
					}
					
					painter.add(mesh);
				}
			}
		}
	}


	//////////////////////////////////////////////////////////////////////////////////
	//// RESTORE BUTTON
	//////////////////////////////////////////////////////////////////////////////////

	pub fn paint_restore_button(self, painter: &Painter, pos: Pos2, size: f32, visuals: &Visuals, focused: bool, state: WidgetState)
	{
		let offset = vec2(size, size) * Self::ICON_MARGIN;

		match self
		{
			Self::Windows => {
				let offset = offset * 1.3;
				let stroke = Self::windows_stroke(visuals, focused);
				let relationship = 0.5;
				
				painter.rect_stroke(Rect::from_min_max(pos - vec2(offset.x, offset.y * relationship), pos + vec2(offset.x * relationship, offset.y)), Rounding::none(), stroke);

				painter.vline(pos.x - offset.x * relationship, (pos.y - offset.y * relationship)..=(pos.y - offset.y), stroke);
				painter.hline((pos.x + offset.x * relationship)..=(pos.x + offset.x), pos.y + offset.y * relationship, stroke);

				painter.hline((pos.x - offset.x * relationship)..=(pos.x + offset.x), pos.y - offset.y, stroke);
				painter.vline(pos.x + offset.x, (pos.y + offset.y * relationship)..=(pos.y - offset.y), stroke);
			}
			Self::Breeze => {
				Self::breeze_background(painter, pos, size, state);

				Self::paint_breeze_arrow(painter, pos - vec2(0., offset.y * 0.55), offset * 1.15, visuals, state);
				Self::paint_breeze_arrow(painter, pos + vec2(0., offset.y * 0.55), offset * vec2(1., -1.) * 1.15, visuals, state);
				// ^^^ The `offset * vec2(1., -1.)` above inverts the arrow ^^^
			}
			Self::Mac => {
				self.paint_maximize_button(painter, pos, size, visuals, focused, state);
			}
		}
	}


	//////////////////////////////////////////////////////////////////////////////////
	//// MINIMIZE BUTTON
	//////////////////////////////////////////////////////////////////////////////////

	pub fn paint_minimize_button(self, painter: &Painter, pos: Pos2, size: f32, visuals: &Visuals, focused: bool, state: WidgetState)
	{
		let offset = vec2(size, size) * Self::ICON_MARGIN;

		match self
		{
			Self::Windows => {				
				painter.hline((pos.x - offset.x)..=(pos.x + offset.x), pos.y, Self::windows_stroke(visuals, focused));
			}
			Self::Breeze => {
				Self::breeze_background(painter, pos, size, state);

				Self::paint_breeze_arrow(painter, pos, offset * vec2(1., -1.) * 1.15, visuals, state)
			}
			Self::Mac => {
				let fill_color = if focused {Self::MAC_MINIMIZE_FILL_COLOR} else {Self::MAC_UNFOCUSED_FILL_COLOR};
				let outline_color = if focused {Self::MAC_MINIMIZE_OUTLINE_COLOR} else {Self::MAC_UNFOCUSED_OUTLINE_COLOR};

				painter.circle(pos, size, fill_color, Stroke::new(0.5, outline_color));

				if state != WidgetState::Inactive && state != WidgetState::Noninteractive {
					painter.hline((pos.x - offset.x)..=(pos.x + offset.x), pos.y, Stroke::new(1.5, outline_color));
				}
			}
		}
	}
}