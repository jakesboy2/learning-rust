use std::mem;

pub struct List {
  head: Link,
}

enum Link {
  Empty,
  More(Box<Node>)
}

struct Node {
  elem: i32,
  next: Link
}

impl List {
  pub fn new() -> Self {
    List { head: Link::Empty }
  }

  pub fn push(&mut self, elem: i32) {
    let new_node = Node {
      elem: elem,
      // You can't simply do next: self.head here, since we are borrowing the value (mutable reference)
      // next: self.head would be giving ownership of self.head to next
      // so self would be like wtf where is my head? 
      // you gave it away to new_node.next?? you only asked if you could borrow it
      //
      // so the reason that replace works is you are putting a new value in self.head (allowed with mutable reference)
      // and replace returns the original value inside of the first param, and sets the value of the first param to the second param
      // so we are giving head back to self, but we changed the value of head temporarily
      next: mem::replace(&mut self.head, Link::Empty)
    };

    // then here we change self.head to the value that we actually want
    self.head = Link::More(Box::new(new_node))
  }

  pub fn pop(&mut self) -> Option<i32> {
    // same deal here, match on the value of self.head
    // match self.head results in compiler error, because match is trying to move the value of self.head to here
    // so we could borrow self.head with &self.head, but notice that it isn't what we end up actually doing
    match mem::replace(&mut self.head, Link::Empty) {
      Link::Empty => None,
      Link::More(node) => {
        // because it would cause an error here
        // you can't assign to a borrowed value, same issue as in .push()
        // but you can assign to a mutable reference
        // 
        // so what we end up doing is using replace to grab the value of self.head
        // and putting Link::Empty in its place until we can assign the correct value to self.head
        self.head = node.next;
        Some(node.elem)
      }
    }
  }
}

// Drop is a built in trait that acts as a destructor, and it is automatically called
// after an objects lifecycle (in this case, when it leaves scope)

// It requires a custom implementation because of some quirkiness with how
// destructing a Node would be, you can't drop it and still refer to the next
// node to go drop that one
impl Drop for List {
  fn drop(&mut self) {
    // empty out the head, store its value in current_link
    let mut current_link = mem::replace(&mut self.head, Link::Empty);

    // `while let` == "do this thing until this pattern doesn't match"
    while let Link::More(mut boxed_node) = current_link {
      // empty out the next value of current_link
      // and return that next value for us to go delete it
      current_link = mem::replace(&mut boxed_node.next, Link::Empty);
        // boxed_node goes out of scope and gets dropped here;
        // but its Node's `next` field has been set to Link::Empty
        // so no unbounded recursion occurs.
    }
  }
}

// cfg(test) only compilers on cargo test (prevents unused import error)
#[cfg(test)]
mod test {
  use super::List;

  #[test]
  fn basics() {
    let mut list = List::new();

    assert_eq!(list.pop(), None);

    list.push(1);
    list.push(2);
    list.push(3);

    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(2));

    list.push(4);
    list.push(5);

    assert_eq!(list.pop(), Some(5));
    assert_eq!(list.pop(), Some(4));

    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);
  }
}
