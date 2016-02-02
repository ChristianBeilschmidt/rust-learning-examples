extern crate binary_search_tree;
extern crate rand;

use binary_search_tree::BinarySearchTree;
use rand::{Rng, SeedableRng, StdRng};

#[test]
fn find_inserts() {
	let mut bst = BinarySearchTree::new();
	
	let seed: &[_] = &[1, 2, 3, 4];
	let mut rng: StdRng = SeedableRng::from_seed(seed);
	
	let mut elements: Vec<i32> = (1..100).collect();
	rng.shuffle(&mut elements);
	
	for e in &elements {
		bst.insert(e.clone());
	}
	
	rng.shuffle(&mut elements);
	
	for e in &elements {
		assert!(bst.search(&e));
	}
}
