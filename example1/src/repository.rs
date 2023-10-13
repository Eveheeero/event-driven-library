use ruva::prelude::*;
use ruva_core::{repository::TRepository, tokio::sync::RwLock};
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::sync::Arc;

pub(super) struct MyRepository<E, A>
where
	E: Executor,
	A: Aggregate,
{
	executor: Arc<RwLock<E>>,
	aggregate: PhantomData<A>,
}

impl<E, A> REventManager<A> for MyRepository<E, A>
where
	E: Executor,
	A: Aggregate,
{
	fn get_events(&mut self) -> VecDeque<Box<dyn Message>> {
		todo!()
	}
	fn set_events(&mut self, events: VecDeque<Box<dyn Message>>) {
		todo!()
	}
	fn event_hook(&mut self, aggregate: &mut A) {
		self.set_events(aggregate.take_events());
	}
}

#[async_trait]
impl<E, A> TRepository<E, A> for MyRepository<E, A>
where
	E: Executor,
	A: Aggregate,
{
	fn new(executor: Arc<RwLock<E>>) -> Self {
		Self { executor, aggregate: todo!() }
	}
	async fn get(&self, aggregate_id: &str) -> Result<A, BaseError> {
		todo!()
	}
	async fn update(&mut self, aggregate: &mut A) -> Result<(), BaseError> {
		todo!()
	}
	async fn add(&mut self, aggregate: &mut A) -> Result<String, BaseError> {
		todo!()
	}
	async fn delete(&self, _aggregate_id: &str) -> Result<(), BaseError> {
		todo!()
	}
}
