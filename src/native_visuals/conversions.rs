use crate::*;

pub trait ToEgui {
	type Output;
	fn to_egui(&self) -> Self::Output;
}
pub trait OptionReplace {
	type Value;
	fn option_replace(&self, value: &mut Self::Value);
}
impl<Value, T: ToEgui<Output = Option<Value>>> OptionReplace for T {
	type Value = Value;

	fn option_replace(&self, to_change: &mut Self::Value) {
		if let Some(value) = self.to_egui() {
			*to_change = value;
		}
	}
}

impl ToEgui for CssColor {
	type Output = Option<Color32>;

	fn to_egui(&self) -> Self::Output {
		let CssColor::RGBA(color) = self.to_rgb().ok()?
				// .expect(&format!("Could not convert css color to egui rgb when trying to get native system theme (value: {self:?})"))
			else {unreachable!()}; // After converting to RGB, this color cannot be anything else

		Some(Color32::from_rgba_premultiplied(color.red, color.green, color.blue, color.alpha))
	}
}

impl ToEgui for BorderSideWidth {
	type Output = f32;

	fn to_egui(&self) -> Self::Output {
		match self {
			BorderSideWidth::Length(width) => width.to_px().unwrap_or(2.),
			BorderSideWidth::Thin => 1.,
			BorderSideWidth::Medium => 2.,
			BorderSideWidth::Thick => 3.,
		}
	}
}

impl ToEgui for LengthPercentage {
	type Output = Option<f32>;

	fn to_egui(&self) -> Self::Output {
		match self {
			// TODO: remove unwraps here
			LengthPercentage::Dimension(len) => len.to_px(),
			LengthPercentage::Percentage(percentage) => Some(percentage.0 * 20.), // Random number
			LengthPercentage::Calc(_) => None,
		}
	}
}

impl<T: ToEgui> ToEgui for Size2D<T> {
	type Output = [<T as ToEgui>::Output; 2];

	fn to_egui(&self) -> Self::Output {
		[self.0.to_egui(), self.1.to_egui()]
	}
}

impl ToEgui for BorderRadius {
	type Output = Option<Rounding>;

	fn to_egui(&self) -> Self::Output {
		Some(Rounding {
			nw: self.top_left.to_egui()[0]?,
			ne: self.top_right.to_egui()[0]?,
			sw: self.bottom_left.to_egui()[0]?,
			se: self.bottom_right.to_egui()[0]?,
		})
	}
}

impl OptionReplace for Border {
	type Value = Stroke;

	fn option_replace(&self, value: &mut Self::Value) {
		value.width = self.width.to_egui();
		self.color.option_replace(&mut value.color);
	}
}