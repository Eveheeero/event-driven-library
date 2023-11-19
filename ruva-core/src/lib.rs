pub use paste::paste;
pub mod aggregate;
pub mod handler;
pub mod message;
pub mod messagebus;
pub mod outbox;
pub mod repository;
pub mod responses;
pub mod snowflake;
pub mod unit_of_work;
pub mod utils;
pub mod prelude {
	pub use crate::aggregate::*;
	pub use crate::handler::*;
	pub use crate::message::*;
	pub use crate::messagebus::*;
	pub use crate::outbox::OutBox;
	pub use crate::responses::*;
	pub use crate::unit_of_work::*;

	pub use crate::utils::*;
	pub use async_trait::async_trait;
	pub use hashbrown::HashMap as HandlerMapper;
	pub use paste::paste;
	pub use serde::{Deserialize, Serialize};
}

pub mod event_macros {
	// pub use crate::init_command_handler;
	// pub use crate::init_event_handler;
	pub use crate::prepare_bulk_insert;
}
