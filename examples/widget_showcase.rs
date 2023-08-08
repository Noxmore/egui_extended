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
	fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame)
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

						ui.add_space(100.);

						let title_bar_buttons = TitleBarButtons::new()
							.kind(TitleBarButtonsKind::Windows)
							.focused(frame.info().window_info.focused)
							.maximized(frame.info().window_info.maximized)
							.rounding_override(1.5)
							.show(ui);

						for (button, response) in title_bar_buttons {
							if response.clicked() {
								match button {
									TitleBarButton::Close => frame.close(),
									TitleBarButton::Maximize => frame.set_maximized(!frame.info().window_info.maximized),
									TitleBarButton::Minimize => frame.set_minimized(true),
								}
							}
						}
					});
				});
			});
		});
	}
}