//! Functions for loading themes from various platforms

use crate::*;

fn walk_style_sheet(rules: &CssRuleList, visuals: &mut Visuals, handlers: &[(&str, &dyn Fn(&Property, &mut Visuals))])
{
	for rule in &rules.0 {
		if let CssRule::Style(rule) = rule {
			for selector in &rule.selectors.0 {
				if let Ok(selector) = selector.to_css_string(PrinterOptions::default())
				{
					for (target, handler) in handlers
					{
						if selector == *target {
							for (property, _) in rule.declarations.iter() {
								handler(property, visuals);
							}

							break;
						}
					}
				}
			}

			// walk_style_sheet(&rule.rules, visuals, &handler);
		}
	}
}

// TODO: Used to remove walk_style_sheet boilerplate, but autocomplete is broken, so i can't use it, thanks, rust
// macro_rules! handlers
// {
// 	{
// 		$visuals:pat,

// 		$(
// 			$target:expr =>
// 			{
// 				$(
// 					$pattern:pat => $setter:expr
// 				),*
// 				$(,)?
// 			}
// 		)*
// 	}
// 		=>
// 	{
// 		&[
// 			$((
// 				$target,
// 				&|property, $visuals|
// 				{
// 					match property {
// 						$($pattern => $setter,)*

// 						_ => {}
// 					}
// 				}
// 			))*
// 		]
// 	};
// }


/// Loads a Gtk theme
pub fn gtk(input: &str, visuals: &mut Visuals)
{
	let Ok(css) = StyleSheet::parse(input, ParserOptions::default()) else {return};

	walk_style_sheet(&css.rules, visuals, 
	&[
		(".background",
		&|property, visuals|
		{
			match property {
				Property::BackgroundColor(color) => color.option_replace(&mut visuals.panel_fill),
				Property::Color(color) => {
					color.option_replace(&mut visuals.widgets.noninteractive.fg_stroke.color);
					color.option_replace(&mut visuals.widgets.noninteractive.bg_stroke.color);
				},

				_ => {}
			}
		}),

		("button",
		&|property, visuals|
		{
			match property {
				Property::Border(border) => {
					visuals.widgets.modify_all(|w| border.option_replace(&mut w.bg_stroke));
				},
				Property::BorderRadius(radius, _) => {
					visuals.widgets.modify_all(|w| radius.option_replace(&mut w.rounding));
				},

				_ => {}
			}
		}),
	]);
}