#![allow(dead_code)]
pub mod pool;
pub mod parser;

pub struct Lagoon {
	pub pools: pool::Pools<i32>,
	parser: parser::Parser
}

impl Lagoon {
	pub fn new() -> Self {
		Self {
			pools: pool::Pools::new(),
			parser: parser::Parser::new()
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
mod tests {
	// #[test]
	// fn it_works() {
	// 	assert_eq!(2 + 2, 4);
	// }
}
