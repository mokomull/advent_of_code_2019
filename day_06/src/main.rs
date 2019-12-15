use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

fn main() {
    do_main("inputs/day_06.txt");
}

fn do_main(path: &str) {
    let input = std::fs::read_to_string(path).expect("Could not read input");

    let root = make_graph(&input);
    let count = count_all_orbits(&root);
    println!("Found {} orbits", count);
    assert_eq!(count, 273985);

    let root = Rc::new(RefCell::new(root));
    let path_to_you = find_path(&root, "YOU").expect("no path to YOU found");
    let path_to_san = find_path(&root, "SAN").expect("no path to SAN found");
    let count = count_uncommon_path_components(path_to_you, path_to_san);
    println!(
        "YOU would have to move {} times to become a sibling of SAN",
        count
    );
    assert_eq!(count, 460);
}

#[derive(Debug)]
struct Node {
    name: String,
    children: Vec<Rc<RefCell<Node>>>,
}

fn get_or_create<'a, 'b>(
    nodes: &'a mut std::collections::HashMap<String, Rc<RefCell<Node>>>,
    name: &'b str,
) -> &'a mut Rc<RefCell<Node>> {
    nodes.entry(name.to_owned()).or_insert_with(|| {
        Rc::new(RefCell::new(Node {
            name: name.to_owned(),
            children: vec![],
        }))
    })
}

fn make_graph(input: &str) -> Node {
    let mut nodes = std::collections::HashMap::<String, Rc<RefCell<Node>>>::new();
    let mut root_name = "".to_owned();

    for line in input.lines() {
        let mut tokens = line.split(')');
        let center = tokens.next().expect("malformed input");
        let orbiter = tokens.next().expect("malformed input");
        assert!(tokens.next().is_none());

        let orbiter_node = get_or_create(&mut nodes, orbiter).clone();
        let mut node = get_or_create(&mut nodes, center).borrow_mut();
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

fn find_path(root: &Rc<RefCell<Node>>, target: &str) -> Option<VecDeque<Rc<RefCell<Node>>>> {
    if root.borrow().name == target {
        return Some(vec![].into());
    }

    for c in &root.borrow().children {
        if let Some(mut path) = find_path(c, target) {
            path.push_front(root.clone());
            return Some(path);
        }
    }

    None
}

fn count_uncommon_path_components(
    mut a: VecDeque<Rc<RefCell<Node>>>,
    mut b: VecDeque<Rc<RefCell<Node>>>,
) -> usize {
    while !a.is_empty() && !b.is_empty() && Rc::ptr_eq(a.front().unwrap(), b.front().unwrap()) {
        a.pop_front();
        b.pop_front();
    }

    a.len() + b.len()
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

    #[test]
    fn find_path() {
        use super::*;
        let root = Rc::new(RefCell::new(make_graph(
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
K)YOU
I)SAN
",
        )));
        let path_to_you = find_path(&root, "YOU").expect("did not find a path to YOU");
        let names_to_you: Vec<_> = path_to_you
            .iter()
            .map(|node| node.borrow().name.clone())
            .collect();
        assert_eq!(names_to_you, vec!["COM", "B", "C", "D", "E", "J", "K"]);

        let path_to_san = find_path(&root, "SAN").expect("did not find a path to SAN");
        assert_eq!(count_uncommon_path_components(path_to_you, path_to_san), 4);
    }

    #[test]
    fn main() {
        super::do_main("../inputs/day_06.txt");
    }
}
