
#[derive(Show,PartialEq)]
struct Node<T> {
    payload: T,
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
}

struct LinkedList<T> {
    root: Option<Node<T>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList::<T> {
            root: None,
        }
    }

    fn add(node: Node<T>) {

        let last_node = &node;

        //loop {
        //    if last_node.payload == None {
        //        last_node = node;
        //        break;
        //    }
        //}
    }
}

#[test]
fn test_node_new() {
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
fn test_node_payload() {
    let node: Node<&str> = Node::new("test");
    assert_eq!(&"test", node.payload().unwrap());
}
