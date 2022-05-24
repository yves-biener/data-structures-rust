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

// into_iter
// trivial wrapper around list for into_iter
pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
	IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
	// access fields of a tuple struct numerically
        self.0.pop()
    }
}

// iter
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

// No lifetime here, List doesn't have any associated lifetimes
impl<T> List<T> {
    // We declare a fresh lifetime here for the *exact* borrow thta creates the
    // iter. Now &self needs to be valid as long as the Iter is around.
    // When using `as_deref` we don't need to provide lifetimes anymore, as the
    // compiler can figure them out by himself.
    // Instead of "hiding" that a struct contains a lifetime, you can use the
    // explicitly elided lifetime syntax `'_`
    pub fn iter(&self) -> Iter<'_, T> {
	// The `as_deref` and `as_defer_mut` functions are stable as of Rust
	// 1.40. Before that you would need to do `map(|node| &**node)` and
	// `map(|node| &mut**node)`. In this case the closure in conjunction
	// with the fact that we have an `Option<&T>` instead of `&T` is a bit
	// too compliacted for the bollow checker to work out, so we need to
	// help it by being explicit.
	Iter { next: self.head.as_deref() }
    }
}

// We *do* have a lifetime here, because Iter has one that we need to define
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    // Self continues to be increadibly hype and amazing
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
	    self.next = node.next.as_deref();
	    &node.elem
	})
    }
}

// iter_mut
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
	IterMut { next: self.head.as_deref_mut() }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item =&'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
	    self.next = node.next.as_deref_mut();
	    &mut node.elem
	})
    }
}
