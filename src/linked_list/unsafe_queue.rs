use std::ptr;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = *mut Node<T>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    pub fn push(&mut self, elem: T) {
        unsafe {
            let new_tail = Box::into_raw(Box::new(Node {
                elem,
                // When you push onto the tail, your next is always None
                next: ptr::null_mut(),
            }));

            // .is_null checks for null, equivalent to checking for None
            if !self.tail.is_null() {
                // if the old tail existed, update it to point to the new tail
                (*self.tail).next = new_tail;
            } else {
                // otherwise, update the head to point to it
                self.head = new_tail;
            }

            self.tail = new_tail;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
	unsafe {
	    if self.head.is_null() {
		None
	    } else {
		// rise from the grave
		let head = Box::from_raw(self.head);
		self.head = head.next;

		if self.head.is_null() {
		    // if head is null the list is empty so we also set tail to
		    // be empty
		    self.tail = ptr::null_mut();
		}
		Some(head.elem)
	    }
	}
    }

    pub fn peek(&self) -> Option<&T> {
	unsafe {
	    self.head.as_ref().map(|node| &node.elem)
	}
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
	unsafe {
	    self.head.as_mut().map(|node| &mut node.elem)
	}
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
	// just pop all elements one by one
	// this is simple but can be inefficient
        while let Some(_) = self.pop() {}
    }
}

// IntoIter
pub struct IntoIter<T>(List<T>);

// Iter
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

// IterMut
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
	IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
	unsafe {
	    Iter { next: self.head.as_ref() }
	}
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
	unsafe {
	    IterMut { next: self.head.as_mut() }
	}
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
	    self.next.map(|node| {
		self.next = node.next.as_ref();
		&node.elem
	    })
	}
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
	    self.next.take().map(|node| {
		self.next = node.next.as_mut();
		&mut node.elem
	    })
	}
    }
}
