use ruva::prelude::*;

#[aggregate]
#[derive(Default, Serialize, Deserialize)]
pub(super) struct MyObjects {
	#[identifier]
	pub(crate) id: usize,
	remain_coffee: usize,
	remain_tea: usize,
	// pub(crate) events: std::collections::VecDeque<std::boxed::Box<dyn Message>>,
}

impl MyObjects {
	pub(crate) fn do_work(&self) {
		// self.raise_event(event)
		// -> uow가 커밋할때 레포지토리에 전달됨, 레포지토리가 나중에 처리함
	}
}
