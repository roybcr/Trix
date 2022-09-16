use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct NodePayload {
    pub frequency: usize,
}

#[derive(Debug)]
pub struct Node {
    pub descendants: HashSet<String>,
    pub payload: NodePayload,
}

#[derive(Debug)]
pub struct Trix {
    pub nodes: HashMap<String, Node>,
    pub size: usize,
}

impl Trix {
    pub fn new() -> Trix {
        const SIZE: usize = 26;
        let initial_nodes = 'a'..='z';

        let mut nodes: HashMap<String, Node> = HashMap::new();

        for ch in initial_nodes {
            let prefix = ch.to_string();
            let descendants: HashSet<String> = HashSet::new();
            let payload: NodePayload = NodePayload { frequency: 0 };
            let node = Node {
                descendants,
                payload,
            };

            nodes.insert(prefix, node);
        }

        for ch2 in 'a'..='z' {
            if let Some(node) = nodes.get_mut(&ch2.to_string()) {
                (*node)
                    .descendants
                    .insert(format!("{}{}", &ch2, ch2).to_string());
            }
        }

        Trix { nodes, size: SIZE }
    }
}
