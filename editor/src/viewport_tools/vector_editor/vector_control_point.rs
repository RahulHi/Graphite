use super::constants::ControlPointType;
use crate::{
	consts::COLOR_ACCENT,
	message_prelude::{DocumentMessage, Message},
};

use graphene::{
	color::Color,
	layers::style::{Fill, PathStyle, Stroke},
	LayerId, Operation,
};

use glam::DVec2;
use std::collections::VecDeque;

/// VectorControlPoint represents any grabbable point, anchor or handle
#[derive(PartialEq, Clone, Debug)]
pub struct VectorControlPoint {
	// The sibling element if this is a handle
	pub position: glam::DVec2,
	// The type of manipulator this point is
	pub manipulator_type: ControlPointType,
	// Can be selected
	pub can_be_selected: bool,
	// Is this point currently selected?
	pub is_selected: bool,
}

impl Default for VectorControlPoint {
	fn default() -> Self {
		Self {
			position: DVec2::ZERO,
			manipulator_type: ControlPointType::Anchor,
			can_be_selected: true,
			is_selected: false,
		}
	}
}

impl VectorControlPoint {
	/// Sets if this point is selected and updates the overlay to represent that
	pub fn set_selected(&mut self, selected: bool, responses: &mut VecDeque<Message>) {
		self.is_selected = selected;
	}
}
