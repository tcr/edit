use std::collections::HashMap;
use doc::*;
use std::borrow::ToOwned;
use std::cmp;

pub struct DelSlice<'a> {
	pub head:Option<DelElement>,
	pub rest:&'a [DelElement],
}

impl<'a> DelSlice<'a> {
	pub fn new(span:&'a DelSpan) -> DelSlice {
		if span.len() == 0 {
			DelSlice {
				head: None,
				rest: &[],
			}
		} else {
			DelSlice {
				head: Some(span[0].clone()),
				rest: &span[1..],
			}
		}
	}

	pub fn next(&mut self) -> DelElement  {
		let res = self.head.clone().unwrap();
		if self.rest.len() == 0 {
			self.head = None;
			self.rest = &[];
		} else {
			self.head = Some(self.rest[0].clone());
			self.rest = &self.rest[1..];
		}
		res
	}

	pub fn get_head(&self) -> DelElement {
		self.head.clone().unwrap()
	}

	pub fn is_done(&self) -> bool {
		self.head.is_none()
	}
}

#[derive(Clone)]
pub struct AddSlice<'a> {
	pub head:Option<AddElement>,
	pub rest:&'a [AddElement],
	stack:AddSliceStack<'a>,
}

type AddSliceStack<'a> = Vec<(Option<AddElement>, &'a [AddElement])>;

impl<'a> AddSlice<'a> {
	fn new_with_stack(span:&'a AddSpan, stack:AddSliceStack<'a>) -> AddSlice<'a> {
		if span.len() == 0 {
			AddSlice {
				head: None,
				rest: &[],
				stack: stack,
			}
		} else {
			AddSlice {
				head: Some(span[0].clone()),
				rest: &span[1..],
				stack: stack,
			}
		}
	}

	fn assign(head:&mut Option<AddElement>, rest:&mut &'a [AddElement], span:&'a AddSpan) {
		if span.len() == 0 {
			*head = None;
			*rest = &[];
		} else {
			*head = Some(span[0].clone());
			*rest = &span[1..];
		}
	}

	pub fn new(span:&'a AddSpan) -> AddSlice {
		AddSlice::new_with_stack(span, vec![])
	}

	pub fn next(&mut self) -> AddElement  {
		let res = self.head.clone().unwrap();
		if self.rest.len() == 0 {
			self.head = None;
			self.rest = &[];
		} else {
			self.head = Some(self.rest[0].clone());
			self.rest = &self.rest[1..];
		}
		res
	}

	pub fn get_head(&self) -> AddElement {
		self.head.clone().unwrap()
	}

	pub fn clone_head(&self) -> Option<AddElement> {
		self.head.clone()
	}

	pub fn is_done(&self) -> bool {
		self.head.is_none()
	}

	// fn assign_last(&mut self, head:AddElement) {
	// 	match head {
	// 		AddGroup(_, span) |
	// 		AddWithGroup(span) => {
	// 			if span.len() == 0 {
	// 				self.head = None;
	// 				self.rest = &[];
	// 			} else {
	// 				self.head = Some(span[0].clone());
	// 				self.rest = &span[1..];
	// 			}
	// 		},
	// 		_ => {
	// 			panic!("Entered wrong thing")
	// 		}
	// 	}
	// }

	// pub fn enter(&self) -> AddSlice {
	// 	let head = self.head.clone();
	// 	let rest = self.rest.clone();
	// 	let mut stack = self.stack.clone();
	// 	stack.push((head, rest));
	// 	let span = match self.head.clone() {
	// 		Some(AddGroup(_, ref span)) |
	// 		Some(AddWithGroup(ref span)) => {
	// 			span.clone()
	// 		},
	// 		_ => {
	// 			panic!("Entered wrong thing")
	// 		}
	// 	};
	// 	let len = span.len();
	// 	if len == 0 {
	// 		AddSlice {
	// 			head: None,
	// 			rest: &[],
	// 			stack: stack
	// 		}
	// 	} else {
	// 		let a = span[0].clone();
	// 		let b = &span[1..];
	// 		AddSlice {
	// 			head: Some(a),
	// 			rest: &b,
	// 			stack: stack
	// 		}
	// 	}
	// }

	// pub fn exit(&'a mut self) {
	// 	let last = self.stack.pop().unwrap();
	// 	self.assign_last();
	// }
}