use std::hash::Hash;

#[derive(Debug, PartialEq)]
pub struct Instruction<T: Eq + Hash> {
	pub index: T,
	pub pool: char
}

#[derive(Debug, PartialEq)]
pub struct Guard<T: Eq + Hash> {
	pub index: T,
	pub negated: bool
}

#[derive(Debug, PartialEq)]
pub struct Closure<T: Eq + Hash> {
	pub parts: Vec<Expression<T>>
}

#[derive(Debug, PartialEq)]
pub struct Loop<T: Eq + Hash> {
	pub guard: Vec<Guard<T>>,
	pub closure: Closure<T>
}

#[derive(Debug, PartialEq)]
pub enum Expression<T: Eq + Hash> {
	Exec,
	Instruction(Instruction<T>),
	Loop(Loop<T>)
}

pub mod factory {
	use std::hash::Hash;
	use super::{Closure, Expression, Guard, Instruction, Loop};

	pub fn new_instruction<T: Eq + Hash>(index: T, pool: char) -> Expression<T> {
		Expression::Instruction(Instruction::<T> {index, pool})
	}

	pub fn new_loop<T: Eq + Hash>(
		guard: Vec<Guard<T>>,
		closure: Closure<T>
	) -> Expression<T> {
		Expression::Loop(Loop::<T> {guard, closure})
	}

	pub fn new_closure<T: Eq + Hash>(parts: Vec<Expression<T>>) -> Closure<T> {
		Closure::<T> {parts}
	}

	pub fn new_guard<T: Eq + Hash>(index: T, negated: bool) -> Guard<T> {
		Guard::<T> {index, negated}
	}

	pub fn new_exec<T: Eq + Hash>() -> Expression<T> {
		Expression::Exec
	}
}
