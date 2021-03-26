use std::{collections::HashMap, hash::Hash};

use super::pool;
use super::interpreter::expression::{Closure, Expression};

pub fn execute<T: Eq + Hash + Copy>(
	memory: &mut HashMap<T, i32>,
	pools: &mut pool::Pools<T>,
	closure: Closure<T>
) {
	for e in closure.parts {
		match e {
			Expression::Exec => {
				for pool in pools.map.values() {
					for index in pool.set.iter() {
						(pool.func)((*memory).entry(*index).or_insert(0));
					}
				}
			},
			Expression::Instruction(i) => {
				match pools.get_pool(i.pool) {
					Ok(p) => p.toggle(i.index),
					Err(s) => panic!("{}", s)
				};
				()
			},
			Expression::Loop(l) => {
				while l.guard.iter().all(|g| {
					let value = memory.get(&g.index).unwrap_or(&0) != &0;
					if g.negated { !value } else { value }
				}) {
					execute(memory, pools, l.closure.clone())
				}
			}
		}
	}
}

#[cfg(test)]
mod from_tree {
	use std::collections::HashMap;

	use crate::lagoon::interpreter::expression::factory as e;
	use crate::lagoon::pool;
	use super::execute;

	#[test]
	fn simple() {
		let mut memory: HashMap<i32, i32> = HashMap::new();
		let mut pools: pool::Pools<i32> = pool::Pools::new();
		pools.add_pool('+', |datum| *datum += 1);
		pools.add_pool('-', |datum| *datum -= 1);

		let program = e::new_closure(vec![
			e::new_instruction(0, '+'),
			e::new_exec()
		]);

		assert_eq!(memory.get(&0).unwrap_or(&0), &0);
		execute(&mut memory, &mut pools, program);
		assert_eq!(memory.get(&0).unwrap_or(&0), &1);
	}

	#[test]
	fn toggle_index() {
		let mut memory: HashMap<i32, i32> = HashMap::new();
		let mut pools: pool::Pools<i32> = pool::Pools::new();
		pools.add_pool('+', |datum| *datum += 1);
		pools.add_pool('-', |datum| *datum -= 1);

		let program = e::new_closure(vec![
			e::new_instruction(0, '+'),
			e::new_instruction(0, '+'),
			e::new_instruction(1, '+'),
			e::new_exec()
		]);

		assert_eq!(memory.get(&0).unwrap_or(&0), &0);
		assert_eq!(memory.get(&1).unwrap_or(&0), &0);
		execute(&mut memory, &mut pools, program);
		assert_eq!(memory.get(&0).unwrap_or(&0), &0);
		assert_eq!(memory.get(&1).unwrap_or(&0), &1);
	}

	#[test]
	fn simple_loop() {
		let mut memory: HashMap<i32, i32> = HashMap::new();
		let mut pools: pool::Pools<i32> = pool::Pools::new();
		pools.add_pool('+', |datum| *datum += 1);
		pools.add_pool('-', |datum| *datum -= 1);

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

		execute(&mut memory, &mut pools, program);
		assert_eq!(memory.get(&0).unwrap_or(&0), &0);
		assert_eq!(memory.get(&1).unwrap_or(&0), &3);
	}

	#[test]
	fn negated_loop() {
		let mut memory: HashMap<i32, i32> = HashMap::new();
		let mut pools: pool::Pools<i32> = pool::Pools::new();
		pools.add_pool('+', |datum| *datum += 1);
		pools.add_pool('-', |datum| *datum -= 1);

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

		execute(&mut memory, &mut pools, program);
		assert_eq!(memory.get(&0).unwrap_or(&0), &2);
		assert_eq!(memory.get(&1).unwrap_or(&0), &1);
	}

	#[test]
	fn nested_loop() {
		let mut memory: HashMap<i32, i32> = HashMap::new();
		let mut pools: pool::Pools<i32> = pool::Pools::new();
		pools.add_pool('+', |datum| *datum += 1);
		pools.add_pool('-', |datum| *datum -= 1);

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

		execute(&mut memory, &mut pools, program);
		assert_eq!(memory.get(&0).unwrap_or(&0), &9);
		assert_eq!(memory.get(&1).unwrap_or(&0), &0);
		assert_eq!(memory.get(&2).unwrap_or(&0), &0);
	}
}
