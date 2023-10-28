mod aggregate;
mod executor;
mod handler;
mod repository;

use aggregate::MyObjects;
use executor::MyDbExecutor;
use handler::order_handler;
use repository::MyRepository;
use ruva::ruva_core::init_event_handler;
use ruva::{prelude::*, ruva_core::init_command};

// define command structure
#[derive(Command, Debug)]
struct Order {
	id: u64,
	item: isize,
	by: Option<String>,
}

init_command!(
  {
	Order: order_handler
  }
);

init_event_handler!({});

// must declared as pub
#[derive(Debug)]
pub struct ServiceResponse {
	success_message: String,
	success_time: std::time::SystemTime,
}

impl ApplicationResponse for ServiceResponse {}

// must declared as pub
#[derive(ApplicationError, Debug)]
pub enum ServiceError {
	OrderError,
	#[stop_sentinel]
	StopSentinel,
	#[stop_sentinel_with_event]
	StopSentinelWithEvent(Box<dyn Message>),
	#[database_error]
	DatabaseError(Box<AnyError>),
	BaseError(BaseError),
}

#[tokio::main]
fn main() {
	let mut message_bus = ruva::ruva_core::messagebus::MessageBus::<ServiceResponse, ServiceError>::new(command_handler(), event_handler());
	let data = Order {
		id: 1,
		item: 1,
		by: Some("me".to_string()),
	};
	let response = message_bus.handle(data).await;
}

/*
repository - 한 aggregate를 가지고 디비 연산을 하거나 하는 객체, aggregate당 하나 존재, transaction unit
handle의 respose는 커멘드에 대한 응답, 이벤드는 중간 연산(파생효과)에 대한 응답ㅌ
*/
