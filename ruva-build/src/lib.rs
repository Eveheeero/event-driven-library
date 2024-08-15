use lsp_server::{Connection, ExtractError, Message, Request, RequestId, Response};
use lsp_types::request::Request as _;
use lsp_types::{request::*, GotoDefinitionResponse, InitializeParams, OneOf, ServerCapabilities};
use serde_json::Value;
use std::error::Error;
use std::io::Write;

pub fn form() {
	eprintln!("starting generic LSP server");

	let (connection, _thread) = run_memory_lsp_server();

	let sender = connection.sender;
	let receiver = connection.receiver;

	sender
		.send(
			Request {
				id: 1.into(),
				method: Initialize::METHOD.into(),
				params: Value::Null,
			}
			.into(),
		)
		.unwrap();

	let msg = receiver.recv().unwrap();
	eprintln!("got msg: {msg:?}");

	eprintln!("shutting down server");
	std::io::stderr().flush().unwrap();
}

fn run_memory_lsp_server() -> (Connection, std::thread::JoinHandle<()>) {
	let (connection, input) = Connection::memory();

	let thread = std::thread::spawn(move || {
		let server_capabilities = serde_json::to_value(&ServerCapabilities {
			definition_provider: Some(OneOf::Left(true)),
			..Default::default()
		})
		.unwrap();
		let initialization_params = match connection.initialize(server_capabilities) {
			Ok(it) => it,
			Err(e) => {
				eprintln!("failed to initialize server: {e:?}");
				return;
			}
		};
		let _ = main_loop(connection, initialization_params);
	});

	(input, thread)
}

fn main_loop(connection: Connection, params: serde_json::Value) -> Result<(), Box<dyn Error + Sync + Send>> {
	let _params: InitializeParams = serde_json::from_value(params).unwrap();
	eprintln!("starting example main loop");
	for msg in &connection.receiver {
		eprintln!("got msg: {msg:?}");
		match msg {
			Message::Request(req) => {
				if connection.handle_shutdown(&req)? {
					return Ok(());
				}
				eprintln!("got request: {req:?}");
				match cast::<GotoDefinition>(req) {
					Ok((id, params)) => {
						eprintln!("got gotoDefinition request #{id}: {params:?}");
						let result = Some(GotoDefinitionResponse::Array(Vec::new()));
						let result = serde_json::to_value(&result).unwrap();
						let resp = Response {
							id,
							result: Some(result),
							error: None,
						};
						connection.sender.send(Message::Response(resp))?;
						continue;
					}
					Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
					Err(ExtractError::MethodMismatch(req)) => req,
				};
				// ...
			}
			Message::Response(resp) => {
				eprintln!("got response: {resp:?}");
			}
			Message::Notification(not) => {
				eprintln!("got notification: {not:?}");
			}
		}
	}
	Ok(())
}

fn cast<R>(req: Request) -> Result<(RequestId, R::Params), ExtractError<Request>>
where
	R: lsp_types::request::Request,
	R::Params: serde::de::DeserializeOwned,
{
	req.extract(R::METHOD)
}
