use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub struct Pool<T: Eq + Hash> {
	pub func: fn(&mut T),
	set: HashSet<T>
}

impl<T: Eq + Hash> Pool<T> {
	pub fn new(func: fn(&mut T)) -> Self { Self { func, set: HashSet::new() } }

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
}

pub struct Pools<T: Eq + Hash> {
	pub count: i32,
	map: HashMap<char, Pool<T>>
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
	pub fn add_pool(&mut self, identifier: char, func: fn(&mut T)) -> &mut Self {
		// if self.map.contains_key(&identifier) { /* Error */ }
		self.map.insert(identifier, Pool::new(func));
		return self;
	}
}
