// TODO: Add support to doc comments in macro. ref: (https://docs.rs/bitflags/latest/src/bitflags/lib.rs.html#557-684)


#[macro_export]
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

#[macro_export]
macro_rules! builder_flag {
	{$(#[$attr:meta])* $field:ident} => {
		$(#[$attr])*
		pub fn $field(mut self) -> Self {
			self.$field = true;
			self
		}
	};
}