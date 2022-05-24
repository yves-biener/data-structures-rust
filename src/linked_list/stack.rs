use std::mem;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

enum Link<T> {
    Empty,
    More(Box<Node<T>>),
}

// wrapper around Link to hide the Link and Node enum/struct from the outside
// because of zero cost abstractions this struct has the same size as that field
pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
	Self { head: Link::<T>::Empty }
    }

    pub fn push(&mut self, elem: T) {
	let new_node = Box::new(Node {
	    elem,
	    // replace the head with an link::Empty temporarly before replacing
	    // it with the new head of the list
	    next: mem::replace(&mut self.head, Link::Empty),
	});
	self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
	match mem::replace(&mut self.head, Link::Empty) {
	    Link::Empty => None,
	    Link::More(node) => {
		// replace this node with the next one
		self.head = node.next;
		// return the value of the current head
		Some(node.elem)
	    },
	}
    }
}


// as the list contains types which implement the Drop Trait the implementation
// for Drop for the List is actually not required. But the automatic handling
// can be bad as it will recursivly call drop on each element in the linked
// list, which can cause a stack overflow.
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

	while let Link::More(mut boxed) = cur_link {
	    cur_link = mem::replace(&mut boxed.next, Link::Empty);
	    // boxed goes out of scope and gets dropped here;
	    // but its Node's `next` field has been set to Link::Empty so no
	    // unbounded recursion occurs
	}
    }
}
