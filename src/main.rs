mod lagoon;

use std::io::{self, BufRead};
use lagoon::Lagoon;

fn main() {
	#[allow(unused_variables)]
	let mut lagoon = Lagoon::new();
	lagoon.gen_pools();

	let stdin = io::stdin();
	for line in stdin.lock().lines() {
		let line = line.expect("Could not read line from standard in");
		println!("{}", line);
	}
}
