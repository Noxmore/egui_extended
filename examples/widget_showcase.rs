#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui::*, *};
use egui_extended::prelude::*;

fn main()
{
	let native_options = NativeOptions
	{
		..Default::default()
	};
	run_native("egui_extended widget showcase", native_options, Box::new(|cc| Box::new(WidgetShowcaseApp::new(cc)))).unwrap();
}

#[derive(Default)]
struct WidgetShowcaseApp
{
	boolean: bool,
}

impl WidgetShowcaseApp
{
	fn new(cc: &CreationContext<'_>) -> Self
	{
		let mut visuals = Visuals::dark();
		
		visuals.widgets.set_all_rounding(Rounding::same(5.));
		// visuals.widgets.noninteractive.fg_stroke = Stroke::new(1., Color32::WHITE);

		cc.egui_ctx.set_visuals(visuals);
		cc.egui_ctx.set_pixels_per_point(1.2);
		Self::default()
	}
}

impl App for WidgetShowcaseApp
{
	fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame)
	{
		CentralPanel::default().show(ctx, |ui|
		{
			ui.vertical_centered(|ui|
			{
				ConnectedTabs::new("example_tabs", false)
					.tab("Tab #1")
					.tab("Second Tab")
					.rounding_override(Rounding::same(15.))
					.show(ui, |ui, i|
				{
					Separator::default().shrink(100.).ui(ui);

					ui.vertical(|ui|
					{
						ui.heading(format!("Tab number: {i}"));

						Switch::new(&mut self.boolean)
							.ui(ui);

						Button::new("Test Button")
							.place()
							.size([120., 20.])
							// .visuals_modifier(&visuals_modifiers::no_background)
							.ui(ui);
					});
				});
			});
		});
	}
}