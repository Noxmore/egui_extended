use crate::*;

pub(crate) mod conversions;
pub(crate) mod theme_loaders;

pub(crate) use conversions::*;

pub(crate) use lightningcss::{stylesheet::{ParserOptions, StyleSheet, PrinterOptions}, rules::{CssRule, CssRuleList}, properties::{Property, border::{BorderSideWidth, Border}, border_radius::BorderRadius}, traits::ToCss, values::{color::CssColor, size::Size2D, length::LengthPercentage}};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone)]
pub struct NativeVisuals
{
	pub toolbar_fill_focused: Color32,
	pub toolbar_fill_unfocused: Color32,
}



#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum NativeThemeColorMode
{
	Light,
	Dark,
	#[default]
	SystemDefault,
}

pub trait NativeTheme
{
	/// Attempts to get visuals that match the native gui style,
	/// if this fails, it will default to regular egui visuals
	/// 
	/// Supported platforms:
	/// - Windows
	/// - Linux (Gnome (probably), Cinnamon)
	/// 
	/// - TODO: Mac, Linux (KDE)
	fn native(color_mode: NativeThemeColorMode) -> Self;
}
impl NativeTheme for Visuals
{
	fn native(color_mode: NativeThemeColorMode) -> Self
	{
		let dark_mode =
			match color_mode {
				NativeThemeColorMode::Light => dark_light::Mode::Light,
				NativeThemeColorMode::Dark => dark_light::Mode::Dark,
				NativeThemeColorMode::SystemDefault => dark_light::detect(),
			} != dark_light::Mode::Light;
		
		let mut visuals = if dark_mode {Visuals::dark()} else {Visuals::light()};
		
		#[cfg(target_os = "windows")]
		{
			
		}
		
		#[cfg(target_os = "linux")]
		{
			let gtk_file = if dark_mode {"gtk-dark.css"} else {"gtk.css"};

			// Attempt to get it via gnome
			let gnome_theme = Command::new("gsettings")
				.args(["get", "org.gnome.desktop.interface", "gtk-theme"])
				.output();

			if let Ok(gnome_theme) = gnome_theme
			{
				if !gnome_theme.status.success() {
					println!("Something went wrong while trying to get system theme from gsettings!");
				}

				// We have the name of the theme! Let's go get it
				let theme_name = gnome_theme.stdout
					.into_iter()
					.map(|b| char::from(b))
					.collect::<String>()
				;

				// My testing showed the output is surrounded by single-quotes
				let theme_name = theme_name.trim().trim_matches('\'');

				let theme_path = PathBuf::from(format!("/usr/share/themes/{theme_name}"));

				let gtk_path = theme_path.join("gtk-3.0/".to_owned() + gtk_file);
				if gtk_path.exists()
				{
					theme_loaders::gtk(
						&fs::read_to_string(&gtk_path)
							.expect(&format!("Could not read theme file {gtk_path:?}")),
						&mut visuals,
					);
					return visuals;
				}
				else {println!("no theme file found")}
			}
		}

		visuals
	}
}
