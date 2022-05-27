use std::cmp::Ordering;
use std::fmt::{self, Debug};
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct LinkedList<T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    // add Ghost to protect pointer
    // we semantically store values of T by-value
    _boo: PhantomData<T>,
}

// allow covariance over T
type Link<T> = Option<NonNull<Node<T>>>;

// PhantomData is required as we use NonNull such that it will always be safe
// and clear to the compiler

struct Node<T> {
    front: Link<T>,
    back: Link<T>,
    elem: T,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            front: None,
            back: None,
            len: 0,
            _boo: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push_front(&mut self, elem: T) {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                front: None,
                back: None,
                elem,
            })));

            if let Some(old) = self.front {
                // Put the new front before the old one
                // get raw pointer out of NonNull using `as_ptr`
                (*old.as_ptr()).front = Some(new);
                (*new.as_ptr()).back = Some(old);
            } else {
                // if there's no front, then we're the empty list and need to
                // set the back too.
                self.back = Some(new);
            }
            self.front = Some(new);
            self.len += 1;
        }
    }

    pub fn push_back(&mut self, elem: T) {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                front: None,
                back: None,
                elem,
            })));

            if let Some(old) = self.back {
                // Put the new back before the old one
                (*old.as_ptr()).back = Some(new);
                (*new.as_ptr()).front = Some(old);
            } else {
                // if there's no back, then we're the empty list and need to set
                // the front too.
                self.front = Some(new);
            }
            self.back = Some(new);
            self.len += 1;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            // only have to do stuff if there is a front node to pop.
            // NOTE: we don't need to mess around with `take` anymore because
            // everything is Copy and there are no dtors that will run if we
            // mess up
            self.front.map(|node| {
                // Bring the Box back to life so we can move out its value and
                // Drop it (Box continues to magically understand this for us)
                let boxed = Box::from_raw(node.as_ptr());
                let result = boxed.elem;

                // Make the next node into the new front
                self.front = boxed.back;
                if let Some(new) = self.front {
                    // Cleanup its reference to the removed node
                    (*new.as_ptr()).front = None;
                } else {
                    // if the front is now null, then this list is now empty!

                    // NOTE: this causes a memory leak as the check can cause a
                    // panic which will leave the list in an invalid state!
                    // debug_assert!(self.len == 1);
                    // to avoid this memory leak and the incorrect state we just
                    // add corresponding test cases instead!

                    self.back = None;
                }
                self.len -= 1;
                result
                // Box gets implicitly freed here, knows there is no T
            })
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        unsafe {
            // only have to do stuff if tere is a back node to pop.
            self.back.map(|node| {
                let boxed = Box::from_raw(node.as_ptr());
                let result = boxed.elem;

                // Make the next node the new back
                self.back = boxed.front;
                if let Some(new) = self.back {
                    // Cleanup its reference to the removed node
                    (*new.as_ptr()).back = None;
                } else {
                    // if the back is now null, then this list is now empty!
                    self.front = None;
                }
                self.len -= 1;
                result
                // Box gets implicitly freed here, knows there is no T
            })
        }
    }

    pub fn front(&self) -> Option<&T> {
        unsafe { self.front.map(|node| &(*node.as_ptr()).elem) }
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        unsafe { self.front.map(|node| &mut (*node.as_ptr()).elem) }
    }

    pub fn back(&self) -> Option<&T> {
        unsafe { self.back.map(|node| &(*node.as_ptr()).elem) }
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        unsafe { self.back.map(|node| &mut (*node.as_ptr()).elem) }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn clear(&mut self) {
        while self.pop_front().is_some() {}
    }
}

// Drop Trait

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        // pop until we have to stop
        while self.pop_front().is_some() {}
    }
}

// +-----------------+
// | Iterator Traits |
// +-----------------+

pub struct Iter<'a, T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _boo: PhantomData<&'a T>,
}

impl<T> LinkedList<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            front: self.front,
            back: self.back,
            len: self.len,
            _boo: PhantomData,
        }
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;

    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.front.map(|node| unsafe {
                self.len -= 1;
                self.front = (*node.as_ptr()).back;
                &(*node.as_ptr()).elem
            })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.back.map(|node| unsafe {
                self.len -= 1;
                self.back = (*node.as_ptr()).front;
                &(*node.as_ptr()).elem
            })
        } else {
            None
        }
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}

pub struct IterMut<'a, T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _boo: PhantomData<&'a mut T>,
}

impl<T> LinkedList<T> {
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            front: self.front,
            back: self.back,
            len: self.len,
            _boo: PhantomData,
        }
    }
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type Item = &'a mut T;

    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.front.map(|node| unsafe {
                self.len -= 1;
                self.front = (*node.as_ptr()).back;
                &mut (*node.as_ptr()).elem
            })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.back.map(|node| unsafe {
                self.len -= 1;
                self.back = (*node.as_ptr()).front;
                &mut (*node.as_ptr()).elem
            })
        } else {
            None
        }
    }
}

impl<'a, T> ExactSizeIterator for IterMut<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}

pub struct IntoIter<T> {
    list: LinkedList<T>,
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter { list: self }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.len, Some(self.list.len))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.list.pop_back()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        self.list.len
    }
}

// +---------------------------------------------+
// | Default Traits, which should be expected... |
// +---------------------------------------------+

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        let mut new_list = Self::new();
        for item in self {
            new_list.push_back(item.clone());
        }
        new_list
    }
}

impl<T> Extend<T> for LinkedList<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push_back(item);
        }
    }
}

impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        list.extend(iter);
        list
    }
}

impl<T: Debug> Debug for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other)
    }
}

impl<T: Eq> Eq for LinkedList<T> {}

impl<T: PartialOrd> PartialOrd for LinkedList<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.iter().partial_cmp(other)
    }
}

impl<T: Ord> Ord for LinkedList<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other)
    }
}

impl<T: Hash> Hash for LinkedList<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.len().hash(state);
        for item in self {
            item.hash(state);
        }
    }
}

// +----------------------+
// | Send and Sync Traits |
// +----------------------+
unsafe impl<T: Send> Send for LinkedList<T> {}
unsafe impl<T: Sync> Sync for LinkedList<T> {}

// IntoIter does not need to implement Send and Sync as it auto derives from
// LinkedList as it just contains a LinkedList which we just declared Send and
// Sync!

unsafe impl<'a, T: Send> Send for Iter<'a, T> {}
unsafe impl<'a, T: Sync> Sync for Iter<'a, T> {}

unsafe impl<'a, T: Send> Send for IterMut<'a, T> {}
unsafe impl<'a, T: Sync> Sync for IterMut<'a, T> {}

// +-----------------------+
// | Cursor Implementation |
// +-----------------------+
pub struct CursorMut<'a, T> {
    cur: Link<T>,
    list: &'a mut LinkedList<T>,
    index: Option<usize>,
}

impl<T> LinkedList<T> {
    pub fn cursor_mut(&mut self) -> CursorMut<T> {
        // because we are using a "ghost" element we start at None
        CursorMut {
            cur: None,
            list: self,
            index: None,
        }
    }
}

impl<'a, T> CursorMut<'a, T> {
    pub fn index(&self) -> Option<usize> {
        self.index
    }

    pub fn move_next(&mut self) {
        if let Some(cur) = self.cur {
            unsafe {
                // we are on a real element, go to its next (back)
                self.cur = (*cur.as_ptr()).back;
                if self.cur.is_some() {
                    *self.index.as_mut().unwrap() += 1;
                } else {
                    // We just walked to the ghost, no more index
                    self.index = None;
                }
            }
        } else if !self.list.is_empty() {
            // we are the ghost, and there is a real front, so move to it
            self.cur = self.list.front;
            self.index = Some(0);
        } else {
            // we are the ghost, but that's the only element.. nothing to do
        }
    }

    pub fn move_prev(&mut self) {
        if let Some(cur) = self.cur {
            unsafe {
                // we are on a real element, go to its previous (front)
                self.cur = (*cur.as_ptr()).front;
                if self.cur.is_some() {
                    *self.index.as_mut().unwrap() -= 1;
                } else {
                    // We just walked to the ghost, no more index
                    self.index = None;
                }
            }
        } else if !self.list.is_empty() {
            // we are the ghost, and there is a real back, so move to it
            self.cur = self.list.back;
            self.index = Some(self.list.len - 1);
        } else {
            // we are the ghost, but that's the only element.. nothing to do
        }
    }

    pub fn current(&mut self) -> Option<&mut T> {
        unsafe { self.cur.map(|node| &mut (*node.as_ptr()).elem) }
    }

    pub fn peek_next(&mut self) -> Option<&mut T> {
        unsafe {
            let next = if let Some(cur) = self.cur {
                // try to follow the cur node's back pointer
                (*cur.as_ptr()).back
            } else {
                // ghost -> try to follow the list's front pointer
                self.list.front
            };

            next.map(|node| &mut (*node.as_ptr()).elem)
        }
    }

    pub fn peek_prev(&mut self) -> Option<&mut T> {
        unsafe {
            let prev = if let Some(cur) = self.cur {
                // try to follow the cur node's front pointer
                (*cur.as_ptr()).front
            } else {
                // ghost -> try to use the list's back pointer
                self.list.back
            };

            prev.map(|node| &mut (*node.as_ptr()).elem)
        }
    }

    pub fn split_before(&mut self) -> LinkedList<T> {
        if let Some(cur) = self.cur {
            // we are pointing at a real element, so the list is non-empty
            debug_assert!(!self.list.is_empty());
            unsafe {
                // current state
                let old_len = self.list.len;
                let old_idx = self.index.unwrap(); // should never panic as the
                                                   // list is non-empty
                let prev = (*cur.as_ptr()).front.take();

                // what self will become
                let new_len = old_len - old_idx;
                let new_front = self.cur;
                let new_idx = Some(0);

                // what the output will become
                let output_len = old_len - new_len;
                let output_front = self.list.front;
                let output_back = prev;

                // Break the links between cur and prev
                if let Some(prev) = prev {
                    (*prev.as_ptr()).back = None;
                }

                // produce the result
                self.list.len = new_len;
                self.list.front = new_front;
                self.index = new_idx;

                LinkedList {
                    front: output_front,
                    back: output_back,
                    len: output_len,
                    _boo: PhantomData,
                }
            }
        } else {
            // we are the "ghost", just replace our list with an empty one.
            // No other state needs to be changed
            std::mem::replace(self.list, LinkedList::new())
        }
    }

    pub fn split_after(&mut self) -> LinkedList<T> {
        if let Some(cur) = self.cur {
            // we are pointing at a real element, so the list is non-empty
            debug_assert!(!self.list.is_empty());
            unsafe {
                // current state
                let old_len = self.list.len;
                let old_idx = self.index.unwrap(); // should never panic as the
                                                   // list is non-empty
                let next = (*cur.as_ptr()).back.take();

                // what self will become
                let new_len = old_idx + 1;
                let new_back = self.cur;
                let new_idx = Some(old_idx);

                // what the output will become
                let output_len = old_len - new_len;
                let output_front = next;
                let output_back = self.list.back;

                // Break the links between cur and prev
                if let Some(next) = next {
                    (*next.as_ptr()).front = None;
                }

                // produce the result
                self.list.len = new_len;
                self.list.back = new_back;
                self.index = new_idx;

                LinkedList {
                    front: output_front,
                    back: output_back,
                    len: output_len,
                    _boo: PhantomData,
                }
            }
        } else {
            // we are the "ghost", just replace our list with an empty one.
            // No other state needs to be changed
            std::mem::replace(self.list, LinkedList::new())
        }
    }

    pub fn splice_before(&mut self, mut input: LinkedList<T>) {
        unsafe {
            if input.is_empty() {
                // Input is empty do nothing.
            } else if let Some(cur) = self.cur {
                // both lists are non-empty
                let in_front = input.front.take().unwrap();
                let in_back = input.back.take().unwrap();

                if let Some(prev) = (*cur.as_ptr()).front {
                    // general case, no boundaries, just internal fixups
                    (*prev.as_ptr()).back = Some(in_front);
                    (*in_front.as_ptr()).front = Some(prev);
                    (*cur.as_ptr()).front = Some(in_back);
                    (*in_back.as_ptr()).back = Some(cur);
                } else {
                    // no prev, we are appending to the front
                    (*cur.as_ptr()).front = Some(in_back);
                    (*in_back.as_ptr()).back = Some(cur);
                    self.list.front = Some(in_front);
                }
                // index moves forward by input length
                *self.index.as_mut().unwrap() += input.len;
            } else if let Some(back) = self.list.back {
                // we are on the ghost but non-empty, append to the back below
                let in_front = input.front.take().unwrap();
                let in_back = input.back.take().unwrap();

                (*back.as_ptr()).back = Some(in_front);
                (*in_front.as_ptr()).front = Some(back);
                self.list.back = Some(in_back);
            } else {
                // we are empty, become the input, remain on the ghost
                std::mem::swap(self.list, &mut input);
            }
        }

        self.list.len += input.len;
        // Not necessary but polite to do
        input.len = 0;
        // input dropped here
    }

    pub fn splice_after(&mut self, mut input: LinkedList<T>) {
        unsafe {
            if input.is_empty() {
                // Input is empty do nothing.
            } else if let Some(cur) = self.cur {
                // both lists are non-empty
                let in_front = input.front.take().unwrap();
                let in_back = input.back.take().unwrap();

                if let Some(next) = (*cur.as_ptr()).back {
                    // general case, no boundaries, just internal fixups
                    (*next.as_ptr()).front = Some(in_back);
                    (*in_back.as_ptr()).back = Some(next);
                    (*cur.as_ptr()).back = Some(in_front);
                    (*in_front.as_ptr()).front = Some(cur);
                } else {
                    // no next, we are appending to the back
                    (*cur.as_ptr()).back = Some(in_front);
                    (*in_front.as_ptr()).front = Some(cur);
                    self.list.back = Some(in_back);
                }
                // index does not change
            } else if let Some(front) = self.list.front {
                // we are on the ghost but non-empty, append to the front
                let in_front = input.front.take().unwrap();
                let in_back = input.back.take().unwrap();

                (*front.as_ptr()).front = Some(in_back);
                (*in_back.as_ptr()).back = Some(front);
                self.list.front = Some(in_front);
            } else {
                // we are empty, become the input, remain on the ghost
                std::mem::swap(self.list, &mut input);
            }
        }

        self.list.len += input.len;
        // Not necessary but polite to do
        input.len = 0;
        // input dropped here
    }

    pub fn insert_after(&mut self, elem: T) {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                front: None,
                back: None,
                elem,
            })));

            if let Some(cur) = self.cur {
                // we have elements in the list
                if let Some(next) = (*cur.as_ptr()).back {
                    // there is also a next element, so we are not at the end of the list
                    (*next.as_ptr()).front = Some(new);
                    (*cur.as_ptr()).back = Some(new);
                    (*new.as_ptr()).front = Some(cur);
                    (*new.as_ptr()).back = Some(next);
                } else {
                    // we are at the end of the list (we are the ghost)
                    (*cur.as_ptr()).back = Some(new);
                    (*new.as_ptr()).front = Some(cur);
                    self.list.back = Some(new);
                }
            } else {
                // we don't have elements in the list, such that `elem` will be the first entry
                self.list.front = Some(new);
                self.list.back = Some(new);
            }
            // increase length
            self.list.len += 1;
        }
    }

    pub fn insert_before(&mut self, elem: T) {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                front: None,
                back: None,
                elem,
            })));

            if let Some(cur) = self.cur {
                // we have elements in the list
                if let Some(prev) = (*cur.as_ptr()).front {
                    // there is also a prev element, so we are not at the beginning of the list
                    (*new.as_ptr()).front = Some(prev);
                    (*new.as_ptr()).back = Some(cur);
                    (*cur.as_ptr()).front = Some(new);
                    (*prev.as_ptr()).back = Some(new);
                } else {
                    // we are at the beginning of the list (we are the ghost)
                    (*cur.as_ptr()).front = Some(new);
                    (*new.as_ptr()).back = Some(cur);
                    self.list.front = Some(new);
                }
            } else {
                // we don't have elements in the list, such that `elem` will be the first entry
                self.list.front = Some(new);
                self.list.back = Some(new);
            }
            // increase length
            self.list.len += 1;
        }
    }

    pub fn remove_after(&mut self) -> Option<T> {
        unsafe {
            if let Some(cur) = self.cur {
                // current list has elements
                if let Some(next) = (*cur.as_ptr()).back {
                    // there is an element to remove
                    // bring back the Box in order to drop it in the end
                    let boxed = Box::from_raw(next.as_ptr());
                    let result = Some(boxed.elem);

                    if let Some(box_next) = boxed.back {
                        // there is an element after the one to remove
                        (*cur.as_ptr()).back = Some(box_next);
                        (*box_next.as_ptr()).front = Some(cur);
                    } else {
                        // there is no element after the one to remove (ghost)
                        (*cur.as_ptr()).back = None;
                        self.list.back = Some(cur);
                    }

                    // decrease length
                    self.list.len -= 1;
                    result
                    // drop boxed afterwards
                } else {
                    // we cannot remove the ghost element so we do nothing
                    None
                }
            } else {
                // we are the ghost
                if let Some(front) = self.list.front {
                    let boxed = Box::from_raw(front.as_ptr());
                    let result = Some(boxed.elem);

                    if let Some(box_next) = boxed.back {
                        // there is a next element which can be the new front of the list
                        self.list.front = Some(box_next);
                        (*box_next.as_ptr()).front = None;
                    } else {
                        // the element we remove is the only element in the list
                        self.list.front = None;
                        self.list.back = None;
                    }

                    // decrease length
                    self.list.len -= 1;
                    result
                    // drop boxed afterwards
                } else {
                    // empty list.. nothing to do
                    None
                }
            }
        }
    }

    pub fn remove_before(&mut self) -> Option<T> {
        unsafe {
            if let Some(cur) = self.cur {
                // current list has elements
                if let Some(prev) = (*cur.as_ptr()).front {
                    // there is an element to remove
                    // bring back the Box in order to drop it in the end
                    let boxed = Box::from_raw(prev.as_ptr());
                    let result = Some(boxed.elem);

                    if let Some(box_prev) = boxed.front {
                        // there is an element before the one to remove
                        (*cur.as_ptr()).front = Some(box_prev);
                        (*box_prev.as_ptr()).back = Some(cur);
                    } else {
                        // there is no element before the one to remove (ghost)
                        (*cur.as_ptr()).front = None;
                        self.list.front = Some(cur);
                    }

                    // decrease length
                    self.list.len -= 1;
                    result
                    // drop boxed afterwards
                } else {
                    // we cannot remove the ghost element so we do nothing
                    None
                }
            } else {
                // we are the ghost
                if let Some(back) = self.list.back {
                    let boxed = Box::from_raw(back.as_ptr());
                    let result = Some(boxed.elem);

                    if let Some(box_prev) = boxed.front {
                        // there is a prev element which can be the new front of the list
                        self.list.back = Some(box_prev);
                        (*box_prev.as_ptr()).back = None;
                    } else {
                        // the element we remove is the only element in the list
                        self.list.front = None;
                        self.list.back = None;
                    }

                    // decrease length
                    self.list.len -= 1;
                    result
                    // drop boxed afterwards
                } else {
                    // empty list.. nothing to do
                    None
                }
            }
        }
    }
}
