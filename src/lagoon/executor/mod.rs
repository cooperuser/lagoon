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
