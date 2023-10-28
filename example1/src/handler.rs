use super::*;

pub(super) async fn order_handler(command: Order, context: AtomicContextManager) -> Result<ServiceResponse, ServiceError> {
	let mut uow: UnitOfWork<MyRepository<MyDbExecutor, MyObjects>, MyDbExecutor, MyObjects> = UnitOfWork::new(context).await;

	// begin은 이미 되어있음

	if command.by.is_none() {
		eprintln!("Cannot deliver order");
		let _ = uow.rollback().await;
		return Err(ServiceError::OrderError);
	}

	let aggregate = uow.repository().get("id").await.unwrap();

	aggregate.do_work();

	uow.commit::<OutBox>();

	todo!()
}
