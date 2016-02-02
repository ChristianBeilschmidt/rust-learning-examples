#[derive(Debug)]
pub struct BinarySearchTree<T> where T: PartialOrd<T> {
	root: Link<T>
}

impl<T> BinarySearchTree<T> where T: PartialOrd<T> {
	pub fn new() -> BinarySearchTree<T> {
		BinarySearchTree { root: Link::Empty }
	}
	
	pub fn insert(&mut self, element: T) -> bool {
		self.root.insert(element)
	}
	
	pub fn search(&self, element: &T) -> bool {
		self.root.search(element)
	}
}

#[derive(Debug)]
struct Node<T> where T: PartialOrd<T> {
	element: T,
	left: Link<T>,
	right: Link<T>
}

impl<T> Node<T> where T: PartialOrd<T> {
	pub fn new(element: T) -> Node<T> {
		Node { element: element, left: Link::Empty, right: Link::Empty }
	}
}

#[derive(Debug)]
enum Link<T> where T: PartialOrd<T> {
	Empty,
	More(Box<Node<T>>)
}

impl<T> Link<T> where T: PartialOrd<T> {
	pub fn insert(&mut self, element: T) -> bool {
		match self {
			&mut Link::Empty => {
				*self = Link::More(Box::new(Node::new(element)));
				true
			},
			&mut Link::More(ref mut node) => {
				if element == node.element {
					false
				} else if element < node.element {
					node.left.insert(element)
				} else {
					node.right.insert(element)
				}
			}
		}
	}
	
	fn search(&self, element: &T) -> bool {
		match self {
			&Link::Empty => false,
			&Link::More(ref node) => {
				if *element == node.element {
					true
				} else if *element < node.element {
					node.left.search(element)
				} else {
					node.right.search(element)
				}
			}
		}
	}
}

fn main() {
	let mut bst = BinarySearchTree::new();
	bst.insert(4);
	bst.insert(3);
	bst.insert(5);
    println!("{:?}", bst);
    
    println!("Lookup 3: {}", bst.search(&3));
    println!("Lookup 6: {}", bst.search(&6));
}
