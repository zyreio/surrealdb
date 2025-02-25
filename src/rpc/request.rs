use crate::rpc::failure::Failure;
use once_cell::sync::Lazy;
use surrealdb::sql::Part;
use surrealdb::sql::{Array, Value};

pub static ID: Lazy<[Part; 1]> = Lazy::new(|| [Part::from("id")]);
pub static METHOD: Lazy<[Part; 1]> = Lazy::new(|| [Part::from("method")]);
pub static PARAMS: Lazy<[Part; 1]> = Lazy::new(|| [Part::from("params")]);

pub struct Request {
	pub id: Option<Value>,
	pub method: String,
	pub params: Array,
}

impl TryFrom<Value> for Request {
	type Error = Failure;
	fn try_from(val: Value) -> Result<Self, Failure> {
		// Fetch the 'id' argument
		let id = match val.pick(&*ID) {
			v if v.is_none() => None,
			v if v.is_null() => Some(v),
			v if v.is_uuid() => Some(v),
			v if v.is_number() => Some(v),
			v if v.is_strand() => Some(v),
			v if v.is_datetime() => Some(v),
			_ => return Err(Failure::INVALID_REQUEST),
		};
		// Fetch the 'method' argument
		let method = match val.pick(&*METHOD) {
			Value::Strand(v) => v.to_raw(),
			_ => return Err(Failure::INVALID_REQUEST),
		};
		// Fetch the 'params' argument
		let params = match val.pick(&*PARAMS) {
			Value::Array(v) => v,
			_ => Array::new(),
		};
		// Return the parsed request
		Ok(Request {
			id,
			method,
			params,
		})
	}
}
