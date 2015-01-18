
struct Node<T> {
    payload: T,
    next: Box<Option<T>>,
    previous: Box<Option<T>>,
}

impl<T> Node<T> {
    pub fn new(payload: T) -> Node<T> {
        Node::<T> {
            payload: payload,
            next: Box::new(None),
            previous: Box::new(None),
        }
    }

    pub fn payload(&self) -> Option<&T> {
        Some(&self.payload)
    }
}

#[test]
fn test_node_new() {
    let expected = Node {
        payload: "test",
        next: Box::new(None),
        previous: Box::new(None),
    };
    let node: Node<&str> = Node::new("test");

    assert_eq!(expected.payload,  node.payload);
    assert_eq!(expected.next,     node.next);
    assert_eq!(expected.previous, node.previous);
}

#[test]
fn test_node_payload() {
    let node: Node<&str> = Node::new("test");
    assert_eq!(&"test", node.payload().unwrap());
}
