mod better_stack;
pub mod doubly_linked_list;
mod persistent_stack;
mod stack;
mod unsafe_queue;

#[allow(dead_code)]
fn assert_properties() {
    fn is_send<T: Send>() {}
    fn is_sync<T: Sync>() {}

    is_send::<doubly_linked_list::LinkedList<i32>>();
    is_sync::<doubly_linked_list::LinkedList<i32>>();

    is_send::<doubly_linked_list::IntoIter<i32>>();
    is_sync::<doubly_linked_list::IntoIter<i32>>();

    is_send::<doubly_linked_list::Iter<i32>>();
    is_sync::<doubly_linked_list::Iter<i32>>();

    is_send::<doubly_linked_list::IterMut<i32>>();
    is_sync::<doubly_linked_list::IterMut<i32>>();

    fn linked_list_covariant<'a, T>(
        x: doubly_linked_list::LinkedList<&'static T>,
    ) -> doubly_linked_list::LinkedList<&'a T> {
        x
    }

    fn iter_covariant<'i, 'a, T>(
        x: doubly_linked_list::Iter<'a, &'static T>,
    ) -> doubly_linked_list::Iter<'i, &'a T> {
        x
    }

    fn into_iter_covariant<'a, T>(
        x: doubly_linked_list::IntoIter<&'static T>,
    ) -> doubly_linked_list::IntoIter<&'a T> {
        x
    }

    // Doctest to prove that IterMut is not covariant to check if it is safe to
    // implement Send and Sync Traits.
    /// ```compile_fail,E0308
    /// use crate::data_structures_rust::linked_list::doubly_linked_list::IterMut;
    ///
    /// fn iter_mut_covariant<'i, 'a, T>(x: IterMut<'i, &'static T>)
    /// -> IterMut<'i, &'a T> { x }
    /// ```
    #[cfg(doctest)]
    fn iter_mut_invariant() {}
}

#[cfg(test)]
mod test_stack {
    use super::*;

    #[test]
    fn test_push() {
        // arrange
        let mut list = stack::List::new();

        // act
        list.push(1);
        list.push(2);
        list.push(3);

        // assert
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);

        // another push, after pop
        // act
        list.push(5);

        // assert
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_pop() {
        // arrange
        let mut list = stack::List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        // act
        let value = list.pop();

        // asert
        assert_eq!(value, Some(3));

        // another pop after push
        // arrange
        list.push(5);

        // act
        let value = list.pop();

        // assert
        assert_eq!(value, Some(5));
    }

    #[test]
    fn test_pop_empty() {
        // arrange
        let mut list = stack::List::<i32>::new();

        // act
        let value = list.pop();

        // assert
        assert_eq!(value, None);
    }
}

#[cfg(test)]
mod test_better_stack {
    use super::*;

    #[test]
    fn test_push() {
        // arrange
        let mut list = better_stack::List::new();

        // act
        list.push(1);
        list.push(2);
        list.push(3);

        // assert
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);

        // another push, after pop
        // act
        list.push(5);

        // assert
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_pop() {
        // arrange
        let mut list = better_stack::List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        // act
        let value = list.pop();

        // asert
        assert_eq!(value, Some(3));

        // another pop after push
        // arrange
        list.push(5);

        // act
        let value = list.pop();

        // assert
        assert_eq!(value, Some(5));
    }

    #[test]
    fn test_pop_empty() {
        // arrange
        let mut list = better_stack::List::<i32>::new();

        // act
        let value = list.pop();

        // assert
        assert_eq!(value, None);
    }

    #[test]
    fn test_peek() {
        // arrange
        let mut list = better_stack::List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        // act
        let value = list.peek();

        // assert
        assert_eq!(value, Some(&{ 3 }));
        assert_eq!(list.pop(), Some(3));

        // another peek after pop
        // act
        let value = list.peek();

        // assert
        assert_eq!(value, Some(&{ 2 }));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
    }

    #[test]
    fn test_peek_empty() {
        // arrange
        let list = better_stack::List::<i32>::new();

        // act
        let value = list.peek();

        // assert
        assert_eq!(value, None);
    }

    #[test]
    fn test_peek_mut() {
        // arrange
        let mut list = better_stack::List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        // act
        if let Some(value) = list.peek_mut() {
            *value = 5;
        }
        // same as above (not sure which I like more/better):
        list.peek_mut().map(|value| {
            *value = 5;
        });

        // assert
        assert_eq!(list.peek_mut(), Some(&mut 5));
        assert_eq!(list.peek(), Some(&{ 5 }));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_into_iter() {
        // arrange
        let mut list = better_stack::List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        // act
        let mut iter = list.into_iter();

        // assert
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter() {
        // arrange
        let mut list = better_stack::List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        // act
        let mut iter = list.iter();

        // assert
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn test_iter_empty() {
        // arrange
        let list = better_stack::List::<i32>::new();

        // act
        let mut iter = list.iter();

        // assert
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_mut() {
        // arrange
        let mut list = better_stack::List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        // act
        let mut iter = list.iter_mut();

        // assert
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }

    #[test]
    fn test_iter_mut_empty() {
        // arrange
        let mut list = better_stack::List::<i32>::new();

        // act
        let mut iter = list.iter_mut();

        // assert
        assert_eq!(iter.next(), None);
    }
}

#[cfg(test)]
mod test_persistent_stack {
    use super::*;

    #[test]
    fn test_prepend() {
        // arrange
        let list = persistent_stack::List::new();

        // act
        let list = list.prepend(1);
        let list = list.prepend(2);
        let list = list.prepend(3);

        // assert
        assert_eq!(list.head(), Some(&3));
        let list = list.tail();
        assert_eq!(list.head(), Some(&2));
        let list = list.tail();
        assert_eq!(list.head(), Some(&1));
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn test_head() {
        // arrange
        let list = persistent_stack::List::new();
        let list = list.prepend(1);
        let list = list.prepend(2);

        // act
        let head = list.head();

        // assert
        assert_eq!(head, Some(&2));

        // and the other head
        // arrange
        let list = list.tail();

        // act
        let head = list.head();

        // assert
        assert_eq!(head, Some(&1));
    }

    #[test]
    fn test_head_empty() {
        // arrange
        let list = persistent_stack::List::<i32>::new();

        // act
        let head = list.head();

        // assert
        assert_eq!(head, None);
    }

    #[test]
    fn test_tail() {
        // arrange
        let list = persistent_stack::List::new();
        let list = list.prepend(1);
        let list = list.prepend(2);

        // act
        let tail = list.tail();

        // assert
        assert_eq!(tail.head(), Some(&1));

        // and the next tail
        // act
        let tail = tail.tail();

        // assert
        assert_eq!(tail.head(), None);
    }

    #[test]
    fn test_tail_empty() {
        // arrange
        let list = persistent_stack::List::<i32>::new();

        // act
        let tail = list.tail();

        // assert
        assert_eq!(tail.head(), None);

        // another tail does not panic
        // act
        let tail = tail.tail();

        // assert
        assert_eq!(tail.head(), None);
    }

    #[test]
    fn test_iter() {
        // arrange
        let list = persistent_stack::List::new();
        let list = list.prepend(1);
        let list = list.prepend(2);
        let list = list.prepend(3);

        // act
        let mut iter = list.iter();

        // assert
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_empty() {
        // arrange
        let list = persistent_stack::List::<i32>::new();

        // act
        let mut iter = list.iter();

        // assert
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}

#[cfg(test)]
mod test_unsafe_queue {
    use super::*;

    #[test]
    fn test_push() {
        // arrange
        let mut list = unsafe_queue::List::new();

        // act
        list.push(1);
        list.push(2);
        list.push(3);

        // assert
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // push some more elements
        // act
        list.push(4);
        list.push(5);

        // assert
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_pop() {
        // arrange
        let mut list = unsafe_queue::List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        // act
        let value = list.pop();

        // assert
        assert_eq!(value.is_some(), true);
        assert_eq!(value, Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_pop_empty() {
        // arrange
        let mut list = unsafe_queue::List::<i32>::new();

        // act
        let value = list.pop();

        // assert
        assert_eq!(value.is_none(), true);
        assert_eq!(value, None);
    }

    #[test]
    fn test_peek() {
        // arrange
        let mut list = unsafe_queue::List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        // act
        let value = list.peek();

        // assert
        assert_eq!(value, Some(&{ 1 }));
        assert_eq!(list.pop(), Some(1));

        // another peek after pop
        // act
        let value = list.peek();

        // assert
        assert_eq!(value, Some(&{ 2 }));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3));
    }

    #[test]
    fn test_peek_empty() {
        // arrange
        let list = unsafe_queue::List::<i32>::new();

        // act
        let value = list.peek();

        // assert
        assert_eq!(value, None);
    }

    #[test]
    fn test_peek_mut() {
        // arrange
        let mut list = unsafe_queue::List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        // act
        if let Some(value) = list.peek_mut() {
            *value = 5;
        }
        // same as above (not sure which I like more/better):
        list.peek_mut().map(|value| {
            *value = 5;
        });

        // assert
        assert_eq!(list.peek_mut(), Some(&mut 5));
        assert_eq!(list.peek(), Some(&{ 5 }));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_into_iter() {
        // arrange
        let mut list = unsafe_queue::List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        // act
        let mut iter = list.into_iter();

        // assert
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter() {
        // arrange
        let mut list = unsafe_queue::List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        // act
        let mut iter = list.iter();

        // assert
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
    }

    #[test]
    fn test_iter_empty() {
        // arrange
        let list = unsafe_queue::List::<i32>::new();

        // act
        let mut iter = list.iter();

        // assert
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_mut() {
        // arrange
        let mut list = unsafe_queue::List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        // act
        let mut iter = list.iter_mut();

        // assert
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
    }

    #[test]
    fn test_iter_mut_empty() {
        // arrange
        let mut list = unsafe_queue::List::<i32>::new();

        // act
        let mut iter = list.iter_mut();

        // assert
        assert_eq!(iter.next(), None);
    }
}

#[cfg(test)]
mod test_doubly_linked_list {
    use super::*;

    #[test]
    fn test_pop_front_empty() {
        // arrange
        let mut list = doubly_linked_list::LinkedList::<i32>::new();

        // act
        let value = list.pop_front();

        // assert
        assert!(value.is_none());
    }

    #[test]
    fn test_pop_front() {
        // arrange
        let mut list = doubly_linked_list::LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // act
        let value = list.pop_front();

        // assert
        assert_eq!(value.is_some(), true);
        assert_eq!(value, Some(3));

        // and the other pop's
        // act & assert
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_push_front() {
        // arrange
        let mut list = doubly_linked_list::LinkedList::new();

        // act
        list.push_front(1);

        // assert
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(1));
        assert!(list.pop_front().is_none());
    }

    #[test]
    fn test_len_empty() {
        // arrange
        let list = doubly_linked_list::LinkedList::<i32>::new();

        // act
        let len = list.len();

        // assert
        assert_eq!(len, 0);
    }

    #[test]
    fn test_len() {
        // arrange
        let mut list = doubly_linked_list::LinkedList::new();
        list.push_front(1);
        list.push_front(1);
        list.push_front(1);

        // act
        let len = list.len();

        // assert
        assert_eq!(len, 3);

        // and another one
        // arrange
        list.pop_front();

        // act
        let len = list.len();

        // assert
        assert_eq!(len, 2);

        // and another one
        // arrange
        list.pop_front();

        // act
        let len = list.len();

        // assert
        assert_eq!(len, 1);

        // and the last one
        // arrange
        list.pop_front();

        // act
        let len = list.len();

        // assert
        assert_eq!(len, 0);

        // just for good measure; another one
        // arrange
        list.pop_back();

        // act
        let len = list.len();

        // assert
        assert_eq!(len, 0);
    }

    #[test]
    fn test_pop_back_empty() {
        // arrange
        let mut list = doubly_linked_list::LinkedList::<i32>::new();

        // act
        let value = list.pop_back();

        // assert
        assert!(value.is_none());
    }

    #[test]
    fn test_pop_back() {
        // arrange
        let mut list = doubly_linked_list::LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // act
        let value = list.pop_back();

        // assert
        assert_eq!(value.is_some(), true);
        assert_eq!(value, Some(1));

        // and the other pop's
        // act & assert
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_push_back() {
        // arrange
        let mut list = doubly_linked_list::LinkedList::new();

        // act
        list.push_back(1);

        // assert
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(1));
        assert!(list.pop_front().is_none());

        // another push back with checking for correct order!
        // act
        list.push_back(1);
        list.push_back(2);

        // assert
        assert_eq!(list.len(), 2);
        assert_eq!(list.front(), Some(&1));
        assert_eq!(list.back(), Some(&2));
    }

    #[test]
    fn test_front_empty() {
        // arrange
        let list = doubly_linked_list::LinkedList::<i32>::new();

        // act
        let value = list.front();

        // assert
        assert!(value.is_none());
    }

    #[test]
    fn test_front() {
        // arrange
        let mut list = doubly_linked_list::LinkedList::<i32>::new();
        list.push_front(1);
        list.push_back(3);

        // act
        let value = list.front();

        // assert
        assert!(value.is_some());
        assert_eq!(value, Some(&1));
    }

    #[test]
    fn test_front_mut_empty() {
        // arrange
        let mut list = doubly_linked_list::LinkedList::<i32>::new();

        // act
        let value = list.front_mut();

        // assert
        assert!(value.is_none());
    }

    #[test]
    fn test_front_mut() {
        // arrange
        let mut list = doubly_linked_list::LinkedList::<i32>::new();
        list.push_front(1);
        list.push_back(3);

        // act
        if let Some(value) = list.front_mut() {
            *value = 5;
        }
        // same as above (not sure which I like more/better):
        list.front_mut().map(|value| {
            *value = 5;
        });

        // assert
        assert_eq!(list.len(), 2);
        assert_eq!(list.back(), Some(&3));
        assert_eq!(list.front(), Some(&5));
    }

    #[test]
    fn test_back_empty() {
        // arrange
        let list = doubly_linked_list::LinkedList::<i32>::new();

        // act
        let value = list.back();

        // assert
        assert!(value.is_none());
    }

    #[test]
    fn test_back() {
        // arrange
        let mut list = doubly_linked_list::LinkedList::<i32>::new();
        list.push_front(1);
        list.push_back(3);

        // act
        let value = list.back();

        // assert
        assert!(value.is_some());
        assert_eq!(value, Some(&3));

        // after poping an element
        // arrange
        list.pop_back();

        // act
        let value = list.back();

        // assert
        assert_eq!(list.len(), 1);
        assert!(value.is_some());
        assert_eq!(value, Some(&1));

        // and the last one
        // arrange
        list.pop_front();

        // act
        let value = list.back();

        // assert
        assert_eq!(list.len(), 0);
        assert!(value.is_none());
    }

    #[test]
    fn test_back_mut_empty() {
        // arrange
        let mut list = doubly_linked_list::LinkedList::<i32>::new();

        // act
        let value = list.back_mut();

        // assert
        assert!(value.is_none());
    }

    #[test]
    fn test_back_mut() {
        // arrange
        let mut list = doubly_linked_list::LinkedList::<i32>::new();
        list.push_front(1);
        list.push_back(3);

        // act
        if let Some(value) = list.back_mut() {
            *value = 5;
        }
        // or
        list.back_mut().map(|value| {
            *value = 5;
        });

        // assert
        assert_eq!(list.len(), 2);
        assert_eq!(list.front(), Some(&1));
        assert_eq!(list.back(), Some(&5));
    }

    #[test]
    fn test_clear_empty() {
        // arrange
        let mut list = doubly_linked_list::LinkedList::<i32>::new();

        // act
        list.clear();

        // assert
        assert_eq!(list.len(), 0);
        assert!(list.front().is_none());
    }

    #[test]
    fn test_clear() {
        // arrange
        let mut list = doubly_linked_list::LinkedList::new();
        list.push_back(1);
        list.push_back(1);
        list.push_back(1);
        // make sure that the length changed
        assert_eq!(list.len(), 3);

        // act
        list.clear();

        // assert
        assert_eq!(list.len(), 0);
        assert!(list.back().is_none());
    }

    #[test]
    fn test_is_empty() {
        // arrange
        let mut list = doubly_linked_list::LinkedList::new();
        list.push_front(1);

        // act
        let value = list.is_empty();

        // assert
        assert!(!value);

        // now clear the list
        // arrange
        list.clear();

        // act
        let value = list.is_empty();

        // assert
        assert!(value);
    }

    #[test]
    fn test_into_iter() {
        // arrange
        let mut list = doubly_linked_list::LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // act
        let mut iter = list.into_iter();

        // assert
        assert_eq!(iter.size_hint(), (3, Some(3)));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter() {
        // arrange
        let mut list = doubly_linked_list::LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // act
        let mut iter = list.iter();

        // assert
        assert_eq!(iter.size_hint(), (3, Some(3)));
        assert_eq!(iter.next_back(), Some(&3));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.size_hint(), (1, Some(1)));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.size_hint(), (0, Some(0)));
    }

    #[test]
    fn test_iter_empty() {
        // arrange
        let list = doubly_linked_list::LinkedList::<i32>::new();

        // act
        let mut iter = list.iter();

        // assert
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_mut() {
        // arrange
        let mut list = doubly_linked_list::LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // act
        let mut iter = list.iter_mut();

        // assert
        assert_eq!(iter.size_hint(), (3, Some(3)));
        assert_eq!(iter.next_back(), Some(&mut 3));
        assert_eq!(iter.size_hint(), (2, Some(2)));
        assert_eq!(iter.next_back(), Some(&mut 2));
        assert_eq!(iter.size_hint(), (1, Some(1)));
        assert_eq!(iter.next_back(), Some(&mut 1));
        assert_eq!(iter.size_hint(), (0, Some(0)));
    }

    #[test]
    fn test_iter_mut_empty() {
        // arrange
        let mut list = doubly_linked_list::LinkedList::<i32>::new();

        // act
        let mut iter = list.iter_mut();

        // assert
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_rev_iter() {
        // arrange
        let mut list = doubly_linked_list::LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // act
        let mut rev_iter = list.iter().rev();

        // assert
        assert_eq!(rev_iter.next(), Some(&1));
        assert_eq!(rev_iter.next(), Some(&2));
        assert_eq!(rev_iter.next(), Some(&3));
        assert_eq!(rev_iter.next(), None);
    }

    #[test]
    fn test_rev_iter_mut() {
        // arrange
        let mut list = doubly_linked_list::LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // act
        let mut rev_iter = list.iter_mut().rev();

        // assert
        assert_eq!(rev_iter.next(), Some(&mut 1));
        assert_eq!(rev_iter.next(), Some(&mut 2));
        assert_eq!(rev_iter.next(), Some(&mut 3));
        assert_eq!(rev_iter.next(), None);
    }

    #[test]
    fn test_rev_into_iter() {
        // arrange
        let mut list = doubly_linked_list::LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // act
        let mut rev_iter = list.into_iter().rev();

        // assert
        assert_eq!(rev_iter.next(), Some(1));
        assert_eq!(rev_iter.next(), Some(2));
        assert_eq!(rev_iter.next(), Some(3));
        assert_eq!(rev_iter.next(), None);
    }

    // +---------------+
    // | Trait testing |
    // +---------------+
    fn list_from<T: Clone>(v: &[T]) -> doubly_linked_list::LinkedList<T> {
        v.iter().map(|x| (*x).clone()).collect()
    }

    #[test]
    fn test_eq_empty() {
        // arrange
        let list: doubly_linked_list::LinkedList<i32> = list_from(&[]);
        let other_list = list_from(&[]);

        // act & assert
        assert_eq!(list, other_list);
    }

    #[test]
    fn test_eq() {
        // arrange
        let mut list: doubly_linked_list::LinkedList<i32> = list_from(&[]);
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        let other_list = list_from(&[1, 2, 3]);

        // act & assert
        assert_eq!(list, other_list);

        // arrange
        list.push_back(5);

        // act & assert
        assert_ne!(list, other_list);
    }

    #[test]
    fn test_ord() {
        // arrange
        let list = list_from(&[]);
        let other_list = list_from(&[1, 2, 3]);

        // act & assert
        assert!(list < other_list);
        assert!(other_list >= list);
    }

    #[test]
    fn test_debug() {
        // arrange
        let list: doubly_linked_list::LinkedList<i32> = (0..10).collect();

        // act
        let debug_str = format!("{:?}", list);

        // assert
        assert_eq!(debug_str, "[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]");

        // and another list content except numbers, let's try str
        // arrange
        let list: doubly_linked_list::LinkedList<&str> = vec!["just", "one", "more", "test"]
            .iter()
            .copied()
            .collect();

        // act
        let debug_str = format!("{:?}", list);

        // assert
        assert_eq!(debug_str, r#"["just", "one", "more", "test"]"#);
    }

    #[test]
    fn test_hashmap() {
        // arrange
        let list: doubly_linked_list::LinkedList<i32> = (0..10).collect();
        let other_list: doubly_linked_list::LinkedList<i32> = (1..11).collect();
        let mut map = std::collections::HashMap::new();

        // act & assert
        assert_eq!(map.insert(list.clone(), "list"), None); // correctly inserted
        assert_eq!(map.insert(other_list.clone(), "other_list"), None); // correctly inserted

        // check for correct length
        assert_eq!(map.len(), 2);

        // get key of corresponding value
        assert_eq!(map.get(&list), Some(&("list")));
        assert_eq!(map.get(&other_list), Some(&("other_list")));

        // remove value and corresponding key
        assert_eq!(map.remove(&list), Some("list"));
        assert_eq!(map.remove(&other_list), Some("other_list"));

        // the map should be empty after removing both keys we added before
        assert!(map.is_empty());
    }

    #[test]
    fn test_cursor_index_empty() {
        // arrange
        let mut list: doubly_linked_list::LinkedList<i32> = doubly_linked_list::LinkedList::new();
        let mut cursor = list.cursor_mut();

        // act
        let idx = cursor.index();

        // assert
        assert!(idx.is_none());

        // arrange
        cursor.move_next();

        // act
        let idx = cursor.index();

        // assert
        assert!(idx.is_none());
    }

    #[test]
    fn test_cursor_index() {
        // arrange
        let mut list: doubly_linked_list::LinkedList<i32> = doubly_linked_list::LinkedList::new();
        list.extend([1, 2, 3, 4, 5, 6]);
        let mut cursor = list.cursor_mut();

        // act
        let idx = cursor.index();

        // assert
        assert!(idx.is_none());

        // arrange
        cursor.move_next();

        // act
        let idx = cursor.index();

        // assert
        assert!(idx.is_some());
        assert_eq!(idx, Some(0));

        // arrange
        cursor.move_prev();
        cursor.move_prev();

        // act
        let idx = cursor.index();

        // assert
        assert!(idx.is_some());
        assert_eq!(idx, Some(5));
    }

    #[test]
    fn test_cursor_peek_next_empty() {
        // arrange
        let mut list: doubly_linked_list::LinkedList<i32> = doubly_linked_list::LinkedList::new();
        let mut cursor = list.cursor_mut();

        // act
        let ghost = cursor.peek_next();

        // assert
        assert!(ghost.is_none());

        // arrange
        cursor.move_next();

        // act
        let ghost = cursor.peek_next();

        // assert
        assert!(ghost.is_none());

        // arrange
        cursor.move_prev();
        cursor.move_prev();

        // act
        let ghost = cursor.current();

        // assert
        assert!(ghost.is_none());
    }

    #[test]
    fn test_cursor_peek_next() {
        // arrange
        let mut list: doubly_linked_list::LinkedList<i32> = doubly_linked_list::LinkedList::new();
        list.extend([1, 2, 3, 4, 5, 6]);
        let mut cursor = list.cursor_mut();

        // act
        let next = cursor.peek_next();

        // assert
        assert!(next.is_some());
        assert_eq!(next, Some(&mut 1));

        // arrange
        cursor.move_prev();

        // act
        let ghost = cursor.peek_next();

        // assert
        assert!(ghost.is_none());
    }

    #[test]
    fn test_cursor_peek_prev_empty() {
        // arrange
        let mut list: doubly_linked_list::LinkedList<i32> = doubly_linked_list::LinkedList::new();
        let mut cursor = list.cursor_mut();

        // act
        let ghost = cursor.peek_prev();

        // assert
        assert!(ghost.is_none());

        // arrange
        cursor.move_prev();

        // act
        let ghost = cursor.peek_prev();

        // assert
        assert!(ghost.is_none());

        // arrange
        cursor.move_prev();
        cursor.move_prev();

        // act
        let ghost = cursor.current();

        // assert
        assert!(ghost.is_none());
    }

    #[test]
    fn test_cursor_peek_prev() {
        // arrange
        let mut list: doubly_linked_list::LinkedList<i32> = doubly_linked_list::LinkedList::new();
        list.extend([1, 2, 3, 4, 5, 6]);
        let mut cursor = list.cursor_mut();

        // act
        let prev = cursor.peek_prev();

        // assert
        assert!(prev.is_some());
        assert_eq!(prev, Some(&mut 6));

        // arrange
        cursor.move_next();

        // act
        let ghost = cursor.peek_prev();

        // assert
        assert!(ghost.is_none());
    }

    #[test]
    fn test_cursor_current_empty() {
        // arrange
        let mut list: doubly_linked_list::LinkedList<i32> = doubly_linked_list::LinkedList::new();
        let mut cursor = list.cursor_mut();

        // act & assert
        let ghost = cursor.current();
        assert!(ghost.is_none());

        // act & assert
        cursor.move_next();
        let ghost = cursor.current();
        assert!(ghost.is_none());

        // act & assert
        cursor.move_prev();
        cursor.move_prev();
        let ghost = cursor.current();
        assert!(ghost.is_none());
    }

    #[test]
    fn test_cursor_current() {
        // arrange
        let mut list: doubly_linked_list::LinkedList<i32> = doubly_linked_list::LinkedList::new();
        list.extend([1, 2, 3, 4, 5, 6]);
        let mut cursor = list.cursor_mut();

        // act & assert
        let ghost = cursor.current();
        assert!(ghost.is_none());

        // act & assert
        cursor.move_next();
        let first = cursor.current();
        assert_eq!(first, Some(&mut 1));

        // act & assert
        cursor.move_next();
        let second = cursor.current();
        assert_eq!(second, Some(&mut 2));
    }

    #[test]
    fn test_cursor_split_before_ghost() {
        // arrange
        let mut list: doubly_linked_list::LinkedList<i32> = doubly_linked_list::LinkedList::new();
        list.extend([1, 2, 3, 4, 5, 6]);
        let mut cursor = list.cursor_mut();

        // act
        let split = cursor.split_before();

        // assert
        assert_eq!(split.len(), 6);
        assert_eq!(split, (1..7).collect());
        assert!(cursor.current().is_none());
        assert!(cursor.index().is_none());

        // after split the cursor has no list content anymore as it has been
        // moved out to `split`
        // arrange
        cursor.move_next();

        // assert
        assert!(cursor.current().is_none());
        assert!(cursor.index().is_none());
    }

    #[test]
    fn test_cursor_split_before_general() {
        // arrange
        let mut list: doubly_linked_list::LinkedList<i32> = doubly_linked_list::LinkedList::new();
        list.extend([1, 2, 3, 4, 5, 6]);
        let mut cursor = list.cursor_mut();
        cursor.move_next();
        cursor.move_next();
        cursor.move_next();

        // act
        let split = cursor.split_before();

        // assert
        assert_eq!(split.len(), 2);
        assert_eq!(split, (1..3).collect());
        // cursor is not on the ghost but on the head of the other split half
        // that remains in the cursor
        assert_eq!(cursor.current(), Some(&mut 3));
        assert_eq!(cursor.index(), Some(0));
        assert_eq!(cursor.peek_next(), Some(&mut 4));
        assert_eq!(cursor.peek_prev(), None);
    }

    #[test]
    fn test_cursor_split_after_ghost() {
        // arrange
        let mut list: doubly_linked_list::LinkedList<i32> = doubly_linked_list::LinkedList::new();
        list.extend([1, 2, 3, 4, 5, 6]);
        let mut cursor = list.cursor_mut();

        // act
        let split = cursor.split_after();

        // assert
        assert_eq!(split.len(), 6);
        assert_eq!(split, (1..7).collect());
        assert!(cursor.current().is_none());
        assert!(cursor.index().is_none());

        // after split the cursor has no list content anymore as it has been
        // moved out to `split`
        // arrange
        cursor.move_next();

        // assert
        assert!(cursor.current().is_none());
        assert!(cursor.index().is_none());
    }

    #[test]
    fn test_cursor_split_after_general() {
        // arrange
        let mut list: doubly_linked_list::LinkedList<i32> = doubly_linked_list::LinkedList::new();
        list.extend([1, 2, 3, 4, 5, 6]);
        let mut cursor = list.cursor_mut();
        cursor.move_next();
        cursor.move_next();
        cursor.move_next();

        // act
        let split = cursor.split_after();

        // assert
        assert_eq!(split.len(), 3);
        assert_eq!(split, (4..7).collect());
        // cursor is not on the ghost but on the head of the other split half
        // that remains in the cursor
        assert_eq!(cursor.current(), Some(&mut 3));
        assert_eq!(cursor.index(), Some(2));
        assert_eq!(cursor.peek_prev(), Some(&mut 2));
        assert_eq!(cursor.peek_next(), None);
    }

    #[test]
    fn test_cursor_splice_before_ghost_empty() {
        // arrange
        let mut list: doubly_linked_list::LinkedList<i32> = doubly_linked_list::LinkedList::new();
        list.extend([1, 2, 3]);
        let other_list: doubly_linked_list::LinkedList<i32> = doubly_linked_list::LinkedList::new();
        let mut cursor = list.cursor_mut();

        // act
        cursor.splice_before(other_list);

        // assert
        assert_eq!(list.len(), 3);
        assert_eq!(list, list_from(&[1, 2, 3]));
    }

    #[test]
    fn test_cursor_splice_before_general_empty() {
        // arrange
        let mut list: doubly_linked_list::LinkedList<i32> = doubly_linked_list::LinkedList::new();
        list.extend([1, 2, 3]);
        let other_list: doubly_linked_list::LinkedList<i32> = doubly_linked_list::LinkedList::new();
        let mut cursor = list.cursor_mut();
        cursor.move_next();
        cursor.move_next();

        // act
        cursor.splice_before(other_list);

        // assert
        assert_eq!(list.len(), 3);
        assert_eq!(list, list_from(&[1, 2, 3]));
    }

    #[test]
    fn test_cursor_splice_before_general_start() {
        // arrange
        let mut list: doubly_linked_list::LinkedList<i32> = doubly_linked_list::LinkedList::new();
        list.extend([1, 2, 3]);
        let mut other_list: doubly_linked_list::LinkedList<i32> =
            doubly_linked_list::LinkedList::new();
        other_list.extend([4, 5, 6]);
        let mut cursor = list.cursor_mut();
        cursor.move_next();
        cursor.move_next();
        assert_eq!(cursor.index(), Some(1));

        // act
        cursor.splice_before(other_list);

        // assert
        assert_eq!(list.len(), 6);
        assert_eq!(list, list_from(&[1, 4, 5, 6, 2, 3]));
    }

    #[test]
    fn test_cursor_splice_before_general_end() {
        // arrange
        let mut list: doubly_linked_list::LinkedList<i32> = doubly_linked_list::LinkedList::new();
        list.extend([1, 2, 3]);
        let mut other_list: doubly_linked_list::LinkedList<i32> =
            doubly_linked_list::LinkedList::new();
        other_list.extend([4, 5, 6]);
        let mut cursor = list.cursor_mut();
        cursor.move_prev();
        assert_eq!(cursor.index(), Some(2));

        // act
        cursor.splice_before(other_list);

        // assert
        assert_eq!(list.len(), 6);
        assert_eq!(list, list_from(&[1, 2, 4, 5, 6, 3]));
    }

    #[test]
    fn test_cursor_splice_before_first() {
        // arrange
        let mut list: doubly_linked_list::LinkedList<i32> = doubly_linked_list::LinkedList::new();
        list.extend([1, 2, 3]);
        let mut other_list: doubly_linked_list::LinkedList<i32> =
            doubly_linked_list::LinkedList::new();
        other_list.extend([4, 5, 6]);
        let mut cursor = list.cursor_mut();
        cursor.move_next();
        assert_eq!(cursor.index(), Some(0));

        // act
        cursor.splice_before(other_list);

        // assert
        assert_eq!(list.len(), 6);
        assert_eq!(list, list_from(&[4, 5, 6, 1, 2, 3]));
    }

    #[test]
    fn test_cursor_splice_before_ghost() {
        // arrange
        let mut list: doubly_linked_list::LinkedList<i32> = doubly_linked_list::LinkedList::new();
        list.extend([1, 2, 3]);
        let mut other_list: doubly_linked_list::LinkedList<i32> =
            doubly_linked_list::LinkedList::new();
        other_list.extend([4, 5, 6]);
        let mut cursor = list.cursor_mut();

        // act
        cursor.splice_before(other_list);

        // assert
        assert_eq!(list.len(), 6);
        assert_eq!(list, (1..7).collect());
    }

    #[test]
    fn test_cursor_splice_after_ghost_empty() {
        // arrange
        let mut list: doubly_linked_list::LinkedList<i32> = doubly_linked_list::LinkedList::new();
        list.extend([1, 2, 3]);
        let other_list: doubly_linked_list::LinkedList<i32> = doubly_linked_list::LinkedList::new();
        let mut cursor = list.cursor_mut();

        // act
        cursor.splice_after(other_list);

        // assert
        assert_eq!(list.len(), 3);
        assert_eq!(list, list_from(&[1, 2, 3]));
    }

    #[test]
    fn test_cursor_splice_after_general_empty() {
        // arrange
        let mut list: doubly_linked_list::LinkedList<i32> = doubly_linked_list::LinkedList::new();
        list.extend([1, 2, 3]);
        let other_list: doubly_linked_list::LinkedList<i32> = doubly_linked_list::LinkedList::new();
        let mut cursor = list.cursor_mut();
        cursor.move_next();
        cursor.move_next();

        // act
        cursor.splice_after(other_list);

        // assert
        assert_eq!(list.len(), 3);
        assert_eq!(list, list_from(&[1, 2, 3]));
    }

    #[test]
    fn test_cursor_splice_after_general_start() {
        // arrange
        let mut list: doubly_linked_list::LinkedList<i32> = doubly_linked_list::LinkedList::new();
        list.extend([1, 2, 3]);
        let mut other_list: doubly_linked_list::LinkedList<i32> =
            doubly_linked_list::LinkedList::new();
        other_list.extend([4, 5, 6]);
        let mut cursor = list.cursor_mut();
        cursor.move_next();
        cursor.move_next();
        assert_eq!(cursor.index(), Some(1));

        // act
        cursor.splice_after(other_list);

        // assert
        assert_eq!(list.len(), 6);
        assert_eq!(list, list_from(&[1, 2, 4, 5, 6, 3]));
    }

    #[test]
    fn test_cursor_splice_after_general_end() {
        // arrange
        let mut list: doubly_linked_list::LinkedList<i32> = doubly_linked_list::LinkedList::new();
        list.extend([1, 2, 3]);
        let mut other_list: doubly_linked_list::LinkedList<i32> =
            doubly_linked_list::LinkedList::new();
        other_list.extend([4, 5, 6]);
        let mut cursor = list.cursor_mut();
        cursor.move_prev();
        assert_eq!(cursor.index(), Some(2));

        // act
        cursor.splice_after(other_list);

        // assert
        assert_eq!(list.len(), 6);
        assert_eq!(list, (1..7).collect());
    }

    #[test]
    fn test_cursor_splice_after_last() {
        // arrange
        let mut list: doubly_linked_list::LinkedList<i32> = doubly_linked_list::LinkedList::new();
        list.extend([1, 2, 3]);
        let mut other_list: doubly_linked_list::LinkedList<i32> =
            doubly_linked_list::LinkedList::new();
        other_list.extend([4, 5, 6]);
        let mut cursor = list.cursor_mut();
        cursor.move_prev();
        assert_eq!(cursor.index(), Some(2));

        // act
        cursor.splice_after(other_list);

        // assert
        assert_eq!(list.len(), 6);
        assert_eq!(list, (1..7).collect());
    }

    #[test]
    fn test_cursor_splice_after_ghost() {
        // arrange
        let mut list: doubly_linked_list::LinkedList<i32> = doubly_linked_list::LinkedList::new();
        list.extend([1, 2, 3]);
        let mut other_list: doubly_linked_list::LinkedList<i32> =
            doubly_linked_list::LinkedList::new();
        other_list.extend([4, 5, 6]);
        let mut cursor = list.cursor_mut();

        // act
        cursor.splice_after(other_list);

        // assert
        assert_eq!(list.len(), 6);
        assert_eq!(list, list_from(&[4, 5, 6, 1, 2, 3]));
    }

    fn check_links<T: Eq + std::fmt::Debug>(list: &doubly_linked_list::LinkedList<T>) {
        let from_front: Vec<_> = list.iter().collect();
        let from_back: Vec<_> = list.iter().rev().collect();
        let re_reved: Vec<_> = from_back.into_iter().rev().collect();

        assert_eq!(from_front, re_reved);
    }

    #[test]
    fn test_iter_splice_after() {
        // arrange
        let mut list: doubly_linked_list::LinkedList<i32> = doubly_linked_list::LinkedList::new();
        list.extend([1, 2, 3]);
        let mut other_list: doubly_linked_list::LinkedList<i32> =
            doubly_linked_list::LinkedList::new();
        other_list.extend([4, 5, 6]);
        let mut cursor = list.cursor_mut();
        cursor.move_next();
        cursor.move_next();
        assert_eq!(cursor.index(), Some(1));

        // act
        cursor.splice_after(other_list);

        // assert
        check_links(&list);
    }

    #[test]
    fn test_iter_splice_before() {
        // arrange
        let mut list: doubly_linked_list::LinkedList<i32> = doubly_linked_list::LinkedList::new();
        list.extend([1, 2, 3]);
        let mut other_list: doubly_linked_list::LinkedList<i32> =
            doubly_linked_list::LinkedList::new();
        other_list.extend([4, 5, 6]);
        let mut cursor = list.cursor_mut();
        cursor.move_next();
        cursor.move_next();
        assert_eq!(cursor.index(), Some(1));

        // act
        cursor.splice_before(other_list);

        // assert
        check_links(&list);
    }
}
