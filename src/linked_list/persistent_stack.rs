// Most important feature:
// Manipulate the tails of lists basically for free

// use Arc instead of Rc to make this inmutable list thread-safe!
use std::rc::Rc;
// use std::sync::Arc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
	List { head: None }
    }

    pub fn prepend(&self, elem: T) -> List<T> {
	List {
	    head: Some(Rc::new(Node {
		elem,
		next: self.head.clone(),
	    }))
	}
    }

    pub fn tail(&self) -> List<T> {
	List {
	    head: self.head.as_ref().and_then(|node| node.next.clone())
	}
    }

    pub fn head(&self) -> Option<&T> {
	self.head.as_ref().map(|node| &node.elem)
    }
}

// because Rc only provides shared access to the containing object (as it could
// be pointed to by other Rc's) we can not mutate the Node inside of the Rc
impl<T> Drop for List<T> {
    fn drop(&mut self) {
	let mut head = self.head.take();
	while let Some(node) = head {
	    if let Ok(mut node) = Rc::try_unwrap(node) {
		// we are the last one to hold a reference to this Rc so we safe
		// to take node and let it go out of scope (and be free)
		head = node.next.take();
	    } else {
		// another instance holds a reference to this node
		// we won't drop any further and stop instead
		break;
	    }
	}
    }
}

// as this list is a inmutable list
// we cannot implement `Iterolter` or `IterMut` for this list

// iter
// identical to better_stack (a mutable list)
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
	Iter { next: self.head.as_deref() }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
	    self.next = node.next.as_deref();
	    &node.elem
	})
    }
}
