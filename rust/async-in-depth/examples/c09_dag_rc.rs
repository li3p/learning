use std::rc::Rc;

// A simple DAG with 4 nodes
//
// NODE1 ──┐
//         ├─> NODE3 ──> NODE4
// NODE2 ──┘

#[allow(dead_code)]
#[derive(Debug)]
struct Node {
    id: usize,
    down: Option<Rc<Node>>,
}

impl Node {
    fn new(id: usize) -> Self {
        Self { id, down: None }
    }

    fn set_down(&mut self, down: Rc<Node>) {
        self.down = Some(down);
    }

    fn get_down(&self) -> Option<Rc<Node>> {
        self.down.as_ref().map(|v| v.clone())
    }
}

fn main() {
    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let mut node3 = Node::new(3);

    let node4 = Node::new(4);
    node3.set_down(Rc::new(node4));

    node1.set_down(Rc::new(node3));
    node2.set_down(node1.get_down().unwrap());

    println!("node1: {:?} node2: {:?}", node1, node2);
}
