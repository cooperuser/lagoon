#![allow(dead_code)]

pub mod pool;
pub mod parser;
pub mod interpreter;
pub mod executor;

use std::{collections::HashMap, hash::Hash};
use interpreter::expression::{Closure, factory::new_closure};

pub struct Lagoon<T: Eq + Hash + Clone> {
	pub memory: HashMap<T, i32>,
	pub pools: pool::Pools<T>,
	pub code: Closure<T>
}

impl<T: Eq + Hash + Clone> Lagoon<T> {
	pub fn new() -> Self {
		Self {
			memory: HashMap::new(),
			pools: pool::Pools::new(),
			code: new_closure(vec![])
		}
	}

	pub fn gen_pools(&mut self) {
		self.pools.add_pool('+', |datum| *datum += 1);
		self.pools.add_pool('-', |datum| *datum -= 1);

		self.pools.add_pool('i', |datum| *datum += 1);
		self.pools.add_pool('o', |datum| print!("{}", datum));
	}
}

#[cfg(test)]
mod manual {
	use super::Lagoon;
	use super::interpreter::expression::factory as e;
	use super::executor::execute;

	#[test]
	fn simple() {
		let mut lagoon: Lagoon<i32> = Lagoon::new();
		lagoon.gen_pools();
		let program = e::new_closure(vec![
			e::new_instruction(0, '+'),
			e::new_exec()
		]);
		lagoon.code = program;
		assert_eq!(lagoon.memory.get(&0).unwrap_or(&0), &0);
		execute(&mut lagoon.memory, &mut lagoon.pools, lagoon.code);
		assert_eq!(lagoon.memory.get(&0).unwrap_or(&0), &1);
	}

	#[test]
	fn toggle_index() {
		let mut lagoon: Lagoon<i32> = Lagoon::new();
		lagoon.gen_pools();
		let program = e::new_closure(vec![
			e::new_instruction(0, '+'),
			e::new_instruction(0, '+'),
			e::new_instruction(1, '+'),
			e::new_exec()
		]);
		lagoon.code = program;
		assert_eq!(lagoon.memory.get(&0).unwrap_or(&0), &0);
		assert_eq!(lagoon.memory.get(&1).unwrap_or(&0), &0);
		execute(&mut lagoon.memory, &mut lagoon.pools, lagoon.code);
		assert_eq!(lagoon.memory.get(&0).unwrap_or(&0), &0);
		assert_eq!(lagoon.memory.get(&1).unwrap_or(&0), &1);
	}

	#[test]
	fn simple_loop() {
		let mut lagoon: Lagoon<i32> = Lagoon::new();
		lagoon.gen_pools();
		let program = e::new_closure(vec![
			e::new_instruction(0, '+'),
			e::new_exec(),
			e::new_exec(),
			e::new_exec(),
			e::new_instruction(0, '+'),
			e::new_instruction(0, '-'),
			e::new_instruction(1, '+'),
			e::new_loop(vec![e::new_guard(0, false)], e::new_closure(vec![
				e::new_exec()
			]))
		]);
		lagoon.code = program;
		execute(&mut lagoon.memory, &mut lagoon.pools, lagoon.code);
		assert_eq!(lagoon.memory.get(&0).unwrap_or(&0), &0);
		assert_eq!(lagoon.memory.get(&1).unwrap_or(&0), &3);
	}

	#[test]
	fn negated_loop() {
		let mut lagoon: Lagoon<i32> = Lagoon::new();
		lagoon.gen_pools();
		let program = e::new_closure(vec![
			e::new_instruction(0, '+'),
			e::new_exec(),
			e::new_exec(),
			e::new_exec(),
			e::new_instruction(0, '+'),
			e::new_instruction(0, '-'),
			e::new_instruction(1, '+'),
			e::new_loop(vec![
				e::new_guard(0, false),
				e::new_guard(1, true)
			], e::new_closure(vec![
				e::new_exec()
			]))
		]);
		lagoon.code = program;
		execute(&mut lagoon.memory, &mut lagoon.pools, lagoon.code);
		assert_eq!(lagoon.memory.get(&0).unwrap_or(&0), &2);
		assert_eq!(lagoon.memory.get(&1).unwrap_or(&0), &1);
	}

	#[test]
	fn nested_loop() {
		let mut lagoon: Lagoon<i32> = Lagoon::new();
		lagoon.gen_pools();
		let program = e::new_closure(vec![
			e::new_instruction(1, '+'),
			e::new_exec(),
			e::new_exec(),
			e::new_exec(),
			e::new_instruction(1, '+'),

			e::new_loop(vec![e::new_guard(1, false)], e::new_closure(vec![
				e::new_instruction(1, '-'),
				e::new_exec(),
				e::new_instruction(1, '-'),

				e::new_instruction(2, '+'),
				e::new_exec(),
				e::new_exec(),
				e::new_exec(),
				e::new_instruction(2, '+'),

				e::new_instruction(2, '-'),
				e::new_loop(vec![e::new_guard(2, false)], e::new_closure(vec![
					e::new_instruction(0, '+'),
					e::new_exec(),
					e::new_instruction(0, '+'),
				])),
				e::new_instruction(2, '-'),
			]))
		]);
		lagoon.code = program;
		execute(&mut lagoon.memory, &mut lagoon.pools, lagoon.code);
		assert_eq!(lagoon.memory.get(&0).unwrap_or(&0), &9);
		assert_eq!(lagoon.memory.get(&1).unwrap_or(&0), &0);
		assert_eq!(lagoon.memory.get(&2).unwrap_or(&0), &0);
	}
}

#[cfg(test)]
mod program {
}
