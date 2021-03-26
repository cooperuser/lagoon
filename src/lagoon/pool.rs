use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub struct Pool<T: Eq + Hash> {
	pub func: fn(&mut i32),
	pub set: HashSet<T>
}

impl<T: Eq + Hash> Pool<T> {
	pub fn new(func: fn(&mut i32)) -> Self { Self { func, set: HashSet::new() } }

	pub fn add(&mut self, item: T) -> bool {
		let success = self.set.contains(&item);
		self.set.insert(item);
		return !success;
	}

	pub fn remove(&mut self, item: T) -> bool {
		let success = self.set.contains(&item);
		self.set.remove(&item);
		return success;
	}

	pub fn has(&self, item: T) -> bool {
		self.set.contains(&item)
	}

	pub fn toggle(&mut self, item: T) -> bool {
		let exists = self.set.contains(&item);
		if exists {
			self.set.remove(&item);
		} else {
			self.set.insert(item);
		}
		return !exists;
	}

	pub fn execute(self, memory: &mut HashMap<T, i32>) {
		for i in self.set { (self.func)((*memory).entry(i).or_insert(0)); }
	}
}

pub struct Pools<T: Eq + Hash> {
	pub count: i32,
	pub map: HashMap<char, Pool<T>>
}

impl<T: Eq + Hash> Pools<T> {
	pub fn new() -> Self { Self { count: 0, map: HashMap::new() } }

	/// ```rust
	/// add_pool(&mut self, c: char, func: fn(&mut T))
	/// ```
	///
	/// Adds a new pool to the program that runs `func` over each memory slot contained in it when an execute command runs.
	///
	/// # Examples
	///
	/// ```rust
	/// use lagoon::Lagoon;
	///
	/// let mut lagoon = Lagoon::new();
	/// lagoon.pools.add_pool('i', |index| *index += 1);
	/// ```
	///
	/// temp
	pub fn add_pool(
		&mut self,
		identifier: char,
		func: fn(&mut i32)
	) -> &mut Self {
		// if self.map.contains_key(&identifier) { /* Error */ }
		self.map.insert(identifier, Pool::new(func));
		return self;
	}

	pub fn get_pool(
		&mut self,
		identifier: char
	) -> Result<&mut Pool<T>, &'static str> {
		match self.map.get_mut(&identifier) {
			Some(p) => Ok(p),
			None => Err("invalid")
		}
	}

	pub fn execute(self, memory: &mut HashMap<T, i32>) {
		for (_, pool) in self.map {
			pool.execute(memory);
		}
	}
}

#[cfg(test)]
mod single_pool {
	use std::collections::HashMap;
	use crate::lagoon::pool::Pool;

	#[test]
	fn single_index() {
		let mut memory: HashMap<i32, i32> = HashMap::new();
		let mut pool: Pool<i32> = Pool::new(|datum| *datum += 1);

		pool.add(0);
		pool.execute(&mut memory);

		assert_eq!(*memory.get(&0).unwrap_or(&0), 1);
	}

	#[test]
	fn multiple_indices() {
		let mut memory: HashMap<i32, i32> = HashMap::new();
		let mut pool: Pool<i32> = Pool::new(|datum| *datum += 1);

		pool.add(0);
		pool.add(2);
		pool.execute(&mut memory);

		assert_eq!(*memory.get(&0).unwrap_or(&0), 1);
		assert_eq!(*memory.get(&1).unwrap_or(&0), 0);
		assert_eq!(*memory.get(&2).unwrap_or(&0), 1);
	}

	#[test]
	fn double_toggle() {
		let mut memory: HashMap<i32, i32> = HashMap::new();
		let mut pool: Pool<i32> = Pool::new(|datum| *datum += 1);

		pool.toggle(0);
		pool.toggle(0);
		pool.execute(&mut memory);

		assert_eq!(*memory.get(&0).unwrap_or(&0), 0);
	}
}

#[cfg(test)]
mod multi_pool {
	use std::collections::HashMap;
	use crate::lagoon::pool::Pool;

	#[test]
	fn single_index_each() {
		let mut memory: HashMap<i32, i32> = HashMap::new();
		let mut pool_a: Pool<i32> = Pool::new(|datum| *datum += 1);
		let mut pool_b: Pool<i32> = Pool::new(|datum| *datum += 1);

		pool_a.add(0);
		pool_b.add(1);
		pool_a.execute(&mut memory);
		pool_b.execute(&mut memory);

		assert_eq!(*memory.get(&0).unwrap_or(&0), 1);
		assert_eq!(*memory.get(&1).unwrap_or(&0), 1);
	}

	#[test]
	fn single_index_overlap() {
		let mut memory: HashMap<i32, i32> = HashMap::new();
		let mut pool_a: Pool<i32> = Pool::new(|datum| *datum += 1);
		let mut pool_b: Pool<i32> = Pool::new(|datum| *datum += 1);

		pool_a.add(0);
		pool_b.add(0);
		pool_a.execute(&mut memory);
		pool_b.execute(&mut memory);

		assert_eq!(*memory.get(&0).unwrap_or(&0), 2);
	}

	#[test]
	fn multiple_indices() {
		let mut memory: HashMap<i32, i32> = HashMap::new();
		let mut pool_a: Pool<i32> = Pool::new(|datum| *datum += 1);
		let mut pool_b: Pool<i32> = Pool::new(|datum| *datum -= 1);

		pool_a.add(0);
		pool_a.add(1);
		pool_b.add(1);
		pool_b.add(2);
		pool_a.execute(&mut memory);
		pool_b.execute(&mut memory);

		assert_eq!(*memory.get(&0).unwrap_or(&0), 1);
		assert_eq!(*memory.get(&1).unwrap_or(&0), 0);
		assert_eq!(*memory.get(&2).unwrap_or(&0), -1);
	}
}
