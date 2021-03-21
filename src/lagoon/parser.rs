use std::str::Chars;

use super::tree::{Guard, Node, Tree};
use super::tree::factory::*;

pub fn parse(input: String) -> Tree {
	let tree: Tree = Tree::empty();
	let line = 1;
	rparse(tree, &mut input.chars(), line)
}

fn rparse(mut tree: Tree, chars: &mut Chars, mut line: i32) -> Tree {
	let mut current: String = String::new();
	while let Some(c) = chars.next() {
		if c == ' ' || c == '\t' { continue; }
		if c == '\n' {
			line += 1;
		} else if c == '{' {
			let guard: Vec<Guard> = find_guard(chars, line);
			let t: Tree = rparse(Tree::empty(), chars, line);
			tree.push(new_loop(guard, t));
		} else if c == '}' {
			return tree;
		} else if c.is_numeric() {
			current.push(c);
		} else {
			current.push(c);
			tree.push(new_symbol(&current, line));
			current.clear();
		}
	}
	tree
}

fn find_guard(chars: &mut Chars<'_>, mut _line: i32) -> Vec<Guard> {
	let mut guards: Vec<Guard> = Vec::new();
	let mut text: String = String::new();
	let mut negated: bool = false;
	while let Some(c) = chars.next() {
		if c == ' ' || c == '\t' { continue; }
		if c == '|' {
			guards.push(Guard { text, negated });
			break;
		}

		if c == '\n' {
			_line += 1;
		} else if c == ',' {
			guards.push(Guard { text, negated });
			text = String::new();
			negated = false;
		} else if c == '!' {
			negated = true;
		} else if !c.is_numeric() {
			// TODO: throw error
		} else {
			text.push(c);
		}
	}
	return guards;
}

pub fn debug(tree: Tree) -> String {
	let mut output = String::new();
	output.push('(');
	let mut first = true;
	for n in tree.nodes {
		if first { first = false; }
		else { output.push_str(", "); }
		match n {
			Node::Symbol(s) => output.push_str(&s.text),
			Node::Loop(l) => output.push_str(&debug(l.tree))
		}
	}
	output.push(')');
	output
}

#[cfg(test)]
mod simple {
	use crate::lagoon::parser::parse;
	use crate::lagoon::tree::factory::*;

	#[test]
	fn single() {
		let input = "0+;".to_string();
		let tree = parse(input);
		let expected = new_tree(vec![
			new_symbol("0+", 1),
			new_exec(1),
		]);
		assert_eq!(tree, expected);
	}

	#[test]
	fn multiple() {
		let input = "0+ 1+ 123+ 321o;".to_string();
		let tree = parse(input);
		let expected = new_tree(vec![
			new_symbol("0+", 1),
			new_symbol("1+", 1),
			new_symbol("123+", 1),
			new_symbol("321o", 1),
			new_exec(1)
		]);
		assert_eq!(tree, expected);
	}
}


#[cfg(test)]
mod advanced {
	use crate::lagoon::parser::parse;
	use crate::lagoon::tree::factory::*;

	#[test]
	fn nested() {
		let input = "0+{0|0-;};".to_string();
		let tree = parse(input);
		let expected = new_tree(vec![
			new_symbol("0+", 1),
			new_loop(
				vec![new_guard("0", false)],
				new_tree(vec![new_symbol("0-", 1), new_exec(1)])
			),
			new_exec(1)
		]);
		assert_eq!(tree, expected);
	}

	#[test]
	fn negated() {
		let input = "0+{!0|0-;};".to_string();
		let tree = parse(input);
		let expected = new_tree(vec![
			new_symbol("0+", 1),
			new_loop(
				vec![new_guard("0", true)],
				new_tree(vec![new_symbol("0-", 1), new_exec(1)])
			),
			new_exec(1)
		]);
		assert_eq!(tree, expected);
	}

	#[test]
	fn multiple_guards() {
		let input = "0+{0,!1|0-;};".to_string();
		let tree = parse(input);
		let expected = new_tree(vec![
			new_symbol("0+", 1),
			new_loop(
				vec![new_guard("0", false), new_guard("1", true)],
				new_tree(vec![new_symbol("0-", 1), new_exec(1)])
			),
			new_exec(1)
		]);
		assert_eq!(tree, expected);
	}
}
