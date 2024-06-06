use std::io::{self, Write};
use std::rc::Rc;
use std::cell::RefCell;

#[allow(warnings)]
struct Node {
    data: i32,
    lcptr: Option<Rc<RefCell<Node>>>,
    rcptr: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(data: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            data,
            lcptr: None,
            rcptr: None,
        }))
    }
}

fn create_binary_tree() -> Option<Rc<RefCell<Node>>> {
    let mut x = String::new();
    print!("Enter a data(-1 for no node):");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut x).unwrap();
    let x: i32 = x.trim().parse().unwrap();
    if x == -1 {
        return None;
    }
    let new_node = Node::new(x);
    print!("Enter the left child of {}: ", x);
    io::stdout().flush().unwrap();
    new_node.borrow_mut().lcptr = create_binary_tree();
    print!("Enter the right child of {}: ", x);
    io::stdout().flush().unwrap();
    new_node.borrow_mut().rcptr = create_binary_tree();
    Some(new_node)
}

fn depth_of_tree(node: &Option<Rc<RefCell<Node>>>) -> i32 {
    match node {
        Some(node) => {
            let node = node.borrow();
            let left_depth = depth_of_tree(&node.lcptr);
            let right_depth = depth_of_tree(&node.rcptr);
            std::cmp::max(left_depth, right_depth) + 1
        }
        None => 0,
    }
}

fn main() {
    let root = create_binary_tree();
    println!("Depth of tree is {}", depth_of_tree(&root));
}