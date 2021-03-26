pub mod expression;

use std::{hash::Hash, str::FromStr};
use super::parser::tree::{Node, Tree};
use self::expression::{Closure, Expression};

pub fn interpret<T: Eq + Hash + Copy + FromStr + Default>(
	tree: Tree
) -> Closure<T> {
	let mut closure: Closure<T> = Closure {parts: Vec::new()};
	for n in tree.nodes {
		closure.parts.push(rinterpret::<T>(&n));
	}
	return closure;
}

fn rinterpret<T: Eq + Hash + Copy + FromStr + Default>(
	node: &Node
) -> Expression<T> {
	use self::expression::factory::*;

	match node {
		Node::Symbol(s) => {
			let mut text = s.text.clone();
			let pool: char = text.pop().unwrap();
			if pool == ';' { return Expression::Exec }
			let index = match text.parse::<T>() {
				Ok(n) => n,
				Err(_) => T::default(),
			};
			new_instruction(index, pool)
		},
		Node::Loop(l) => {
			let guard = l.guard.iter().map(|g| {
				let index = match g.text.parse::<T>() {
					Ok(n) => n,
					Err(_) => T::default(),
				};
				new_guard(index, g.negated)
			}).collect();
			let closure = new_closure(l.tree.nodes.iter()
				.map(rinterpret::<T>).collect());
			new_loop(guard, closure)
		}
	}
}

#[cfg(test)]
mod simple {
	use super::interpret;
	use crate::lagoon::parser::tree::factory as t;
	use super::expression::factory as e;

	#[test]
	fn single() {
		let tree = t::new_tree(vec![
			t::new_symbol("0+", 1),
			t::new_exec(1),
		]);
		let actual = interpret::<i32>(tree);
		let expected = e::new_closure(vec![
			e::new_instruction(0, '+'),
			e::new_exec()
		]);
		assert_eq!(actual, expected);
	}

	#[test]
	fn multiple() {
		let tree = t::new_tree(vec![
			t::new_symbol("0+", 1),
			t::new_symbol("1+", 1),
			t::new_exec(1),
		]);
		let actual = interpret::<i32>(tree);
		let expected = e::new_closure(vec![
			e::new_instruction(0, '+'),
			e::new_instruction(1, '+'),
			e::new_exec()
		]);
		assert_eq!(actual, expected);
	}
}

#[cfg(test)]
mod advanced {
	use super::interpret;
	use crate::lagoon::parser::tree::factory as t;
	use super::expression::factory as e;

	#[test]
	fn nested() {
		let tree = t::new_tree(vec![
			t::new_symbol("0+", 1),
			t::new_loop(
				vec![t::new_guard("0", false)],
				t::new_tree(vec![t::new_symbol("0-", 1), t::new_exec(1)])
			),
			t::new_exec(1)
		]);
		let actual = interpret::<i32>(tree);
		let expected = e::new_closure(vec![
			e::new_instruction(0, '+'),
			e::new_loop(
				vec![e::new_guard(0, false)],
				e::new_closure(vec![
					e::new_instruction(0, '-'),
					e::new_exec()
				])
			),
			e::new_exec()
		]);
		assert_eq!(actual, expected);
	}

	#[test]
	fn negated() {
		let tree = t::new_tree(vec![
			t::new_symbol("0+", 1),
			t::new_loop(
				vec![t::new_guard("0", true)],
				t::new_tree(vec![t::new_symbol("0-", 1), t::new_exec(1)])
			),
			t::new_exec(1)
		]);
		let actual = interpret(tree);
		let expected = e::new_closure(vec![
			e::new_instruction(0, '+'),
			e::new_loop(
				vec![e::new_guard(0, true)],
				e::new_closure(vec![
					e::new_instruction(0, '-'),
					e::new_exec()
				])
			),
			e::new_exec()
		]);
		assert_eq!(actual, expected);
	}

	#[test]
	fn multiple_guards() {
		let tree = t::new_tree(vec![
			t::new_symbol("0+", 1),
			t::new_loop(
				vec![t::new_guard("0", false), t::new_guard("1", true)],
				t::new_tree(vec![t::new_symbol("0-", 1), t::new_exec(1)])
			),
			t::new_exec(1)
		]);
		let actual = interpret(tree);
		let expected = e::new_closure(vec![
			e::new_instruction(0, '+'),
			e::new_loop(
				vec![e::new_guard(0, false), e::new_guard(1, true)],
				e::new_closure(vec![
					e::new_instruction(0, '-'),
					e::new_exec()
				])
			),
			e::new_exec()
		]);
		assert_eq!(actual, expected);
	}
}
