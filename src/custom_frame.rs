use crate::*;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum TitleBarButtons
{
	Windows,
	Mac,
	/// Linux using the breeze theme
	Breeze,
}

impl TitleBarButtons
{
	
}