extern crate binary_search_tree;

use binary_search_tree::BinarySearchTree;

fn main() {
	let mut bst = BinarySearchTree::new();
	bst.insert(4);
	bst.insert(3);
	bst.insert(5);
    println!("{:?}", bst);
    
    println!("Lookup 3: {}", bst.search(&3));
    println!("Lookup 6: {}", bst.search(&6));
}
