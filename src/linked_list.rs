mod stack;
mod better_stack;

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
	assert_eq!(value, Some(&{3}));
	assert_eq!(list.pop(), Some(3));

	// another peek after pop
	// act
	let value = list.peek();

	// assert
	assert_eq!(value, Some(&{2}));
	assert_eq!(list.pop(), Some(2));
	assert_eq!(list.pop(), Some(1));
    }

    #[test]
    fn test_peek_empty() {
	// arrange
	let mut list = better_stack::List::<i32>::new();

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
	assert_eq!(list.peek(), Some(&{5}));
	assert_eq!(list.pop(), Some(5));
	assert_eq!(list.pop(), Some(2));
	assert_eq!(list.pop(), Some(1));
	assert_eq!(list.pop(), None);
    }
}
