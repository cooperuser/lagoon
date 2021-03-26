use std::hash::Hash;

#[derive(Debug, PartialEq, Clone)]
pub struct Instruction<T: Eq + Hash + Clone> {
	pub index: T,
	pub pool: char
}

#[derive(Debug, PartialEq, Clone)]
pub struct Guard<T: Eq + Hash + Clone> {
	pub index: T,
	pub negated: bool
}

#[derive(Debug, PartialEq, Clone)]
pub struct Closure<T: Eq + Hash + Clone> {
	pub parts: Vec<Expression<T>>
}

#[derive(Debug, PartialEq, Clone)]
pub struct Loop<T: Eq + Hash + Clone> {
	pub guard: Vec<Guard<T>>,
	pub closure: Closure<T>
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression<T: Eq + Hash + Clone> {
	Exec,
	Instruction(Instruction<T>),
	Loop(Loop<T>)
}

pub mod factory {
	use std::hash::Hash;
	use super::{Closure, Expression, Guard, Instruction, Loop};

	pub fn new_instruction<T: Eq + Hash + Clone>(
		index: T,
		pool: char
	) -> Expression<T> {
		Expression::Instruction(Instruction::<T> {index, pool})
	}

	pub fn new_loop<T: Eq + Hash + Clone>(
		guard: Vec<Guard<T>>,
		closure: Closure<T>
	) -> Expression<T> {
		Expression::Loop(Loop::<T> {guard, closure})
	}

	pub fn new_closure<T: Eq + Hash + Clone>(
		parts: Vec<Expression<T>>
	) -> Closure<T> {
		Closure::<T> {parts}
	}

	pub fn new_guard<T: Eq + Hash + Clone>(index: T, negated: bool) -> Guard<T> {
		Guard::<T> {index, negated}
	}

	pub fn new_exec<T: Eq + Hash + Clone>() -> Expression<T> {
		Expression::Exec
	}
}
