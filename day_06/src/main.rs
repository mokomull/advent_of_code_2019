use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Default)]
struct Node {
    children: Vec<Rc<RefCell<Node>>>,
}

fn make_graph(input: &str) -> Node {
    let mut nodes = std::collections::HashMap::<String, Rc<RefCell<Node>>>::new();
    let mut root_name = "".to_owned();

    for line in input.lines() {
        let mut tokens = line.split(')');
        let center = tokens.next().expect("malformed input");
        let orbiter = tokens.next().expect("malformed input");
        assert!(tokens.next().is_none());

        if root_name.is_empty() || root_name == orbiter {
            root_name = center.to_owned();
        }

        let orbiter_node = nodes.entry(orbiter.to_owned()).or_default().clone();
        let mut node = nodes.entry(center.to_owned()).or_default().borrow_mut();
        node.children.push(orbiter_node);
    }

    let root_node = nodes
        .remove(&root_name)
        .expect("root was not found in nodes");
    // Since this is the root, the refcount of root_node should be exactly 1 -- no other node is pointing at it.
    Rc::try_unwrap(root_node)
        .expect("unexpected reference in the bagging area")
        .into_inner()
}

fn count_all_orbits(root: &Node) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod test {
    #[test]
    fn examples() {
        let root = super::make_graph(
            "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
",
        );
        assert_eq!(super::count_all_orbits(&root), 42);
    }
}
