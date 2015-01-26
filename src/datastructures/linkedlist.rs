use std::mem;

/// Lots of thanks to reem and Sharp for help with this from the #rust channel

#[derive(Show,PartialEq)]
struct Node<T> {
    pub payload: T,
    next: Option<Box<Node<T>>>,
    prev: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(payload: T) -> Node<T> {
        Node::<T> {
            payload: payload,
            next: None,
            prev: None,
        }
    }

    pub fn payload(&self) -> Option<&T> {
        Some(&self.payload)
    }

    pub fn insert_after(&mut self, mut new: Box<Node<T>>) -> Option<Box<Node<T>>> {
        mem::swap(&mut new.next, &mut self.next);
        mem::replace(&mut self.next, Some(new))
    }

    pub fn insert_before(&mut self, mut new: Box<Node<T>>) -> Option<Box<Node<T>>> {
        mem::swap(&mut new.prev, &mut self.prev);
        mem::replace(&mut self.prev, Some(new))
    }
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
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

#[cfg(test)]
mod tests {
    use super::*;

}
