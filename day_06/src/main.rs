use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    do_main("inputs/day_06.txt");
}

fn do_main(path: &str) {
    let input = std::fs::read_to_string(path).expect("Could not read input");

    let root = make_graph(&input);
    let count = count_all_orbits(&root);
    println!("Found {} orbits", count);
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

        let orbiter_node = nodes.entry(orbiter.to_owned()).or_default().clone();
        let mut node = nodes.entry(center.to_owned()).or_default().borrow_mut();
        node.children.push(orbiter_node);
    }

    'outer: for (name, node) in &nodes {
        for other_node in nodes.values() {
            for child in &other_node.borrow().children {
                if Rc::ptr_eq(&child, node) {
                    continue 'outer;
                }
            }
        }
        // No other node has this node as a child, so it must be the root.
        root_name = name.to_owned();
        break;
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
    fn count_children(this: &Node, level: usize) -> usize {
        let mut count = level; // count this node for every parent that it has

        for c in &this.children {
            count += count_children(&c.borrow(), level + 1);
        }

        count
    }

    count_children(root, 0)
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

    #[test]
    fn circular_graph() {
        let caught = std::panic::catch_unwind(|| super::make_graph("A)B\nB)A\n"));
        assert_eq!(
            caught
                .unwrap_err()
                .downcast_ref::<String>()
                .expect("should have panicked with a String"),
            "root was not found in nodes"
        );
    }
}
