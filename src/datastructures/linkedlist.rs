
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

    pub fn add(&mut self, payload: T) {

        match self.head {
            None            => self.head = Some(Box::new(Node::new(payload))),
            Some(ref ptr)   => {
                let mut node_ptr = ptr;
                loop {

                    match node_ptr.next {
                        None => {
                            println!("this is the last");
                            /*node_ptr.next = Some(Box::new(Node {
                                payload: payload,
                                next: None,
                                prev: None, //Some(Box::new(&node_ptr)),
                            }));*/
                            break;
                        },
                        Some(ref ptr)   => {
                            node_ptr = ptr;
                        },
                    }

                }
            },
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

    #[test]
    fn new_linked_list() {
        let mut list: super::LinkedList<String> = super::LinkedList::new();
        list.add("first".to_string());
        println!("Value added: {}", list.head.unwrap().payload);
        //assert_eq!(list.head.unwrap().payload, "first".to_string());
    }

    #[test]
    fn multiple_nodes() {
        let mut list: super::LinkedList<String> = super::LinkedList::new();
        list.add("first".to_string());
        list.add("second".to_string());
        //println!("Value added: {}", list.head.unwrap().payload);
        //assert_eq!(list.head.unwrap().payload, "first".to_string());
    }

}
