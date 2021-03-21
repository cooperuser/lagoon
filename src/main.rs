mod lagoon;

// use std::io::{self, BufRead};
// use lagoon::Lagoon;
use lagoon::parser::{debug, parse};

fn main() {
	// #[allow(unused_variables)]
	// let mut lagoon = Lagoon::new();
	// lagoon.gen_pools();
	// Parser::parse("this is a test	asdf".to_string());
	let input = "0+ 1+ 123+ 321o;".to_string();
	let tree = parse(input);

	println!("{}", debug(tree));

	// let stdin = io::stdin();
	// for line in stdin.lock().lines() {
	// 	let line = line.expect("Could not read line from standard in");
	// 	println!("{}", line);
	// }
}
