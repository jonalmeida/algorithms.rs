//! A LinkedList that lets you insert elements from the front or the end of the list.
//! TBD: Add the ability to insert in-between a Node.
//! Lots of thanks to reem and Sharp for help with this from the #rust channel

use std::mem;

/// A Node is the basic unit in a LinkedList which stores the payload of type T.
#[derive(Debug,PartialEq)]
struct Node<T> {
    /// The payload of type T that is wrapped aorund a Node.
    pub payload: T,
    /// An Option of a boxed Node<T> that would point to the next Node<T>.
    next: Option<Box<Node<T>>>,
    /// An Option of a boxed Node<T> that would point to the previous Node<T>.
    prev: Option<Box<Node<T>>>,
}

impl<T> Node<T> {

    pub fn new(payload: T) -> Node<T> {
        //! Creates a instance of a `Node` with `next` and `prev` point to a Option::Node.
        Node::<T> {
            payload: payload,
            next: None,
            prev: None,
        }
    }

    pub fn payload(&mut self) -> Option<&T> {
        //! Access to a payload of Node.
        Some(&self.payload)
    }

    pub fn insert_after(&mut self, mut new: Box<Node<T>>) -> Option<Box<Node<T>>> {
        //! Takes a new boxed Node<T> and swaps it with any existing Node after the current one.
        //! We replace any Option::None or Option::Some with a new Option<Box<Node<T>>>
        mem::swap(&mut new.next, &mut self.next);
        mem::replace(&mut self.next, Some(new))
    }

    pub fn insert_before(&mut self, mut new: Box<Node<T>>) -> Option<Box<Node<T>>> {
        //! Takes a new boxed Node<T> and swaps it with any existing Node after the current one.
        //! We replace any Option::None or Option::Some with a new Option<Box<Node<T>>>
        mem::swap(&mut new.prev, &mut self.prev);
        mem::replace(&mut self.prev, Some(new))
    }

    pub fn next(&mut self) -> Option<&mut T> {
        //! Goes to the next Node and returns an Option to it.
        let Node {
            ref mut payload,
            ref mut prev,
            ref mut next,
        } = *self;

        match *next {
            Some(ref mut next) => {
                mem::swap(&mut next.payload, payload);

                let mut next = &mut **next;     // Cannot completely understand this bit yet.
                mem::swap(&mut next.next, &mut next.prev);

                mem::swap(&mut next.prev, prev);
            },
            None    => return None,
        }
        mem::swap(prev, next);
        Some(payload)
    }
}

/// A LinkedList is the struct that you would need to use to create a list of elements.
/// It contains a pointer to the first and last node in the list represented as `head and `tail`
/// respectively.
pub struct LinkedList<T> {
    /// First element in the list.
    head: Option<Box<Node<T>>>,
    /// Last element in the list.
    tail: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        /// Create a new LinkedList to contain a payloads with type T.
        LinkedList::<T> {
            head: None,
            tail: None,
        }
    }
}

#[test]
fn node_new() {
    let expected = Node {
        payload: "test",
        next: None,
        prev: None,
    };

    let node: Node<&str> = Node::new("test");

    assert_eq!(expected.payload,  node.payload);
    assert_eq!(expected.next,     node.next);
    assert_eq!(expected.prev,     node.prev);
}

#[test]
fn node_payload() {
    let node: Node<&str> = Node::new("test");
    assert_eq!(&"test", node.payload().unwrap());
}

#[test]
fn node_insert_after() {
    let mut node: Node<&str> = Node::new("one");
    node.insert_after(Box::new(Node::new("two")));
    assert_eq!("two", node.next.unwrap().payload);

}

#[test]
fn node_insert_before() {
    let mut node: Node<&str> = Node::new("two");
    node.insert_before(Box::new(Node::new("one")));
    assert_eq!("one", node.prev.unwrap().payload);
}

#[cfg(test)]
mod tests {
    use super::*;

}
