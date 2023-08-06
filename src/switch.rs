use crate::*;

/// Copied from the Widget gallery example code
pub struct Switch<'a>
{
	on: &'a mut bool,
}

impl<'a> Switch<'a>
{
	pub fn new(on: &'a mut bool) -> Self {
		Self {
			on,
		}
	}
}

impl<'a> Widget for Switch<'a> {
	fn ui(self, ui: &mut Ui) -> Response
	{
		// TODO: Support resizing
		let desired_size = ui.spacing().interact_size.y * vec2(2.0, 1.0);
		let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());
		if response.clicked() {
			*self.on = !*self.on;
			response.mark_changed();
		}
		response.widget_info(|| WidgetInfo::selected(WidgetType::Checkbox, *self.on, ""));
	
		if ui.is_rect_visible(rect) {
			let how_on = ui.ctx().animate_bool(response.id, *self.on);
			let visuals = ui.style().interact_selectable(&response, *self.on);
			let rect = rect.expand(visuals.expansion);
			let radius = 0.5 * rect.height();
			ui.painter()
				.rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
			let circle_x = lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
			let center = pos2(circle_x, rect.center().y);
			ui.painter()
				.circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
		}
	
		response
	}
}