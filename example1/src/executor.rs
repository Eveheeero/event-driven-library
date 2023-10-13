use ruva_core::tokio::sync::RwLock;
use ruva::prelude::*;
use std::sync::Arc;

pub(super) struct MyDbExecutor {
	connection: SqlConnection
}

#[async_trait]
impl Executor for MyDbExecutor {
	async fn new() -> Arc<RwLock<Self>> {
		Arc::new(RwLock::new(MyDbExecutor {}))
	}
	async fn begin(&mut self) -> Result<(), BaseError> {
		// 디비 연결후 초기작업
		todo!()
	}
	async fn commit(&mut self) -> Result<(), BaseError> {
		// 디비 마무리작업
		todo!()
	}
	async fn rollback(&mut self) -> Result<(), BaseError> {
		// 디비 취소작업
		todo!()
	}
}
