use crate::*;

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
		#[allow(unreachable_code)] Self::Windows
	}
}

impl TitleBarButtonsKind
{
	pub fn show_close_button(self, ui: &mut Ui, rect: Rect) -> Response
	{
		match self
		{
			Self::Windows => {}
			Self::Mac => {}
			Self::Breeze => {}
		}

		todo!()
	}
}


pub struct TitleBarButtons
{
	kind: TitleBarButtonsKind,
	maximized: bool,

	close_button: bool,
	maximize_button: bool,
	minimize_button: bool,
}

impl TitleBarButtons
{
	pub fn new() -> Self {
		Self {
			kind: Default::default(),
			maximized: false,

			close_button: true,
			maximize_button: true,
			minimize_button: true,
		}
	}
}

impl Widget for TitleBarButtons {
	fn ui(self, ui: &mut Ui) -> Response
	{
		todo!()
	}
}