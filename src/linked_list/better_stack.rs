use std::mem;

// using option instead of an own enum enables us to use all the available
// functions on options we don't have to implement!
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

// wrapper around Link to hide the Link and Node enum/struct from the outside
// because of zero cost abstractions this struct has the same size as that field
pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
	Self { head: None }
    }

    pub fn push(&mut self, elem: T) {
	let new_node = Box::new(Node {
	    elem,
	    // the trick using mem::replace is actually pretty common and for
	    // option there is actually a method `take`
	    next: self.head.take(),
	});
	self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
	// instead of using the pattern matching use `map` instead
	self.head.take().map(|node| {
	    // replace this node with the next one
	    self.head = node.next;
	    // return the value of the current head
	    node.elem // no need to wrap this into an Option / Some
	})
    }

    pub fn peek(&self) -> Option<&T> {
	self.head.as_ref().map(|node| {
	    &node.elem
	})
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
	self.head.as_mut().map(|node| {
	    &mut node.elem
	})
    }
}


// as the list contains types which implement the Drop Trait the implementation
// for Drop for the List is actually not required. But the automatic handling
// can be bad as it will recursivly call drop on each element in the linked
// list, which can cause a stack overflow.
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();

	while let Some(mut boxed) = cur_link {
	    cur_link = boxed.next.take();
	    // boxed goes out of scope and gets dropped here; but its Node's
	    // `next` field has been set to None so no unbounded recursion
	    // occurs
	}
    }
}
