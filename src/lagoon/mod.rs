#![allow(dead_code)]

pub mod pool;
pub mod parser;
pub mod interpreter;
pub mod executor;

use std::{collections::HashMap, hash::Hash, str::FromStr};
use interpreter::expression::{Closure, factory::new_closure};

pub struct Lagoon<T: Eq + Hash + Clone> {
	pub memory: HashMap<T, i32>,
	pub pools: pool::Pools<T>,
	pub code: Closure<T>
}

impl<T: Eq + Hash + Clone + Default + FromStr> Lagoon<T> {
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

	pub fn append(&mut self, input: String) {
		let tree = parser::parse(input);
		let mut closure = interpreter::interpret(tree);
		self.code.parts.append(&mut closure.parts);
	}

	pub fn append_raw(&mut self, input: &str) {
		self.append(input.to_string());
	}
}

#[cfg(test)]
mod full {
	use super::Lagoon;
	use super::executor::execute;

	#[test]
	fn simple() {
		let mut lagoon: Lagoon<i32> = Lagoon::new();
		lagoon.gen_pools();
		lagoon.append_raw("0+;");
		assert_eq!(lagoon.memory.get(&0).unwrap_or(&0), &0);
		execute(&mut lagoon.memory, &mut lagoon.pools, lagoon.code);
		assert_eq!(lagoon.memory.get(&0).unwrap_or(&0), &1);
	}

	#[test]
	fn toggle_index() {
		let mut lagoon: Lagoon<i32> = Lagoon::new();
		lagoon.gen_pools();
		lagoon.append_raw("0+0+1+;");
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
		lagoon.append_raw("0+;;;0+0-1+{0|;}");
		execute(&mut lagoon.memory, &mut lagoon.pools, lagoon.code);
		assert_eq!(lagoon.memory.get(&0).unwrap_or(&0), &0);
		assert_eq!(lagoon.memory.get(&1).unwrap_or(&0), &3);
	}

	#[test]
	fn negated_loop() {
		let mut lagoon: Lagoon<i32> = Lagoon::new();
		lagoon.gen_pools();
		lagoon.append_raw("0+;;;0+0-1+{0,!1|;}");
		execute(&mut lagoon.memory, &mut lagoon.pools, lagoon.code);
		assert_eq!(lagoon.memory.get(&0).unwrap_or(&0), &2);
		assert_eq!(lagoon.memory.get(&1).unwrap_or(&0), &1);
	}

	#[test]
	fn nested_loop() {
		let mut lagoon: Lagoon<i32> = Lagoon::new();
		lagoon.gen_pools();
		lagoon.append_raw("1+;;;1+{1|1-;1- 2+;;;2+ 2-{2|0+;0+}2-}");
		execute(&mut lagoon.memory, &mut lagoon.pools, lagoon.code);
		assert_eq!(lagoon.memory.get(&0).unwrap_or(&0), &9);
		assert_eq!(lagoon.memory.get(&1).unwrap_or(&0), &0);
		assert_eq!(lagoon.memory.get(&2).unwrap_or(&0), &0);
	}

	#[test]
	fn multiplication() {
		let mut lagoon: Lagoon<i32> = Lagoon::new();
		lagoon.gen_pools();
		lagoon.append_raw("0+1+;;;0+;1+{0|0-;0-2+1-{1|;}2+1-2-1+3+{2|;}2-1+3+}");
		execute(&mut lagoon.memory, &mut lagoon.pools, lagoon.code);
		assert_eq!(lagoon.memory.get(&0).unwrap_or(&0), &0);
		assert_eq!(lagoon.memory.get(&1).unwrap_or(&0), &4);
		assert_eq!(lagoon.memory.get(&2).unwrap_or(&0), &0);
		assert_eq!(lagoon.memory.get(&3).unwrap_or(&0), &12);
	}
}
