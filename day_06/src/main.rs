use std::rc::Rc;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Default)]
struct Node {
    children: Vec<Rc<Node>>,
}

fn make_graph(input: &str) -> Node {
    let mut nodes = std::collections::HashMap::<String, Rc<Node>>::new();
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
        let node = nodes.entry(center.to_owned()).or_default();
        Rc::get_mut(node)
            .expect("unintended reference to a node")
            .children
            .push(orbiter_node);
    }

    Rc::try_unwrap(nodes.get(&root_name).expect("root not found").clone())
        .expect("couldn't unwrap out of the hashmap")
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
