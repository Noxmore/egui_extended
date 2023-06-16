// TODO: Doesn't have field autocomplete
// #[macro_export]
// macro_rules! eframe_native {
// 	{$title:expr => $app:path $(,$field:ident : $value:expr)* $(,)?} =>
// 	// {$title:expr => $app:path $(,$field:pat)* $(,)?} =>
// 	{
// 		let native_options = NativeOptions
// 		{
// 			$($field : $value,)*
// 			// $($field,)*
// 			..Default::default()
// 		};
// 		run_native($title, native_options, Box::new(|cc| Box::new($app(cc)))).unwrap();
// 	};
// }