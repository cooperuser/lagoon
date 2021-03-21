#[derive(Debug, PartialEq)]
pub struct Symbol {
	pub text: String,
	pub line: i32
}

#[derive(Debug, PartialEq)]
pub struct Guard {
	pub text: String,
	pub negated: bool
}

#[derive(Debug, PartialEq)]
pub struct Loop {
	pub guard: Vec<Guard>,
	pub tree: Tree
}

#[derive(Debug, PartialEq)]
pub struct Tree {
	pub nodes: Vec<Node>
}

impl Tree {
	pub fn new(nodes: Vec<Node>) -> Self { Self { nodes } }
	pub fn empty() -> Self { Tree::new(Vec::new()) }
	pub fn push(&mut self, node: Node) { self.nodes.push(node) }
}

#[derive(Debug, PartialEq)]
pub enum Node {
	Symbol(Symbol),
	Loop(Loop)
}

pub mod factory {
	use super::{Guard, Loop, Node, Symbol, Tree};

	pub fn new_tree(nodes: Vec<Node>) -> Tree {
		Tree::new(nodes)
	}

	pub fn new_symbol(text: &str, line: i32) -> Node {
		Node::Symbol(Symbol {text: text.to_string(), line})
	}

	pub fn new_loop(guard: Vec<Guard>, tree: Tree) -> Node {
		Node::Loop(Loop {guard, tree})
	}

	pub fn new_guard(text: &str, negated: bool) -> Guard {
		Guard {text: text.to_string(), negated}
	}

	pub fn new_exec(line: i32) -> Node {
		Node::Symbol(Symbol {text: ";".to_string(), line})
	}
}
