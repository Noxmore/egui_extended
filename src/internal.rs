/// Used for reducing boilerplate on builder set macros
macro_rules! builder_set {
	{$(#[$attr:meta])* $field:ident : $type:ty} => {
		$(#[$attr])*
		pub fn $field(mut self, $field: $type) -> Self {
			self.$field = $field;
			self
		}
	};
	{$(#[$attr:meta])* $fn:ident => $field:ident : $type:ty} => {
		$(#[$attr])*
		pub fn $fn(mut self, value: $type) -> Self {
			self.$field = value;
			self
		}
	};

	{$(#[$attr:meta])* $field:ident : $type:ty => $get:expr} => {
		$(#[$attr])*
		pub fn $field(mut self, $field: $type) -> Self {
			self.$field = $get;
			self
		}
	};
	{$(#[$attr:meta])* $fn:ident => $field:ident : $type:ty => $get:expr} => {
		$(#[$attr])*
		pub fn $fn(mut self, $field: $type) -> Self {
			self.$field = $get;
			self
		}
	};
}
pub(crate) use builder_set;


// Un-used atm, could be useful later tho
// #[macro_export]
// macro_rules! builder_flag {
// 	{$(#[$attr:meta])* $field:ident} => {
// 		$(#[$attr])*
// 		pub fn $field(mut self) -> Self {
// 			self.$field = true;
// 			self
// 		}
// 	};
// }