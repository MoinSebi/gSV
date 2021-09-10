use std::collections::{HashMap, HashSet};
use gfaR::Gfa;


pub struct count_node{
    pub ncount: HashMap<String, u32>,
}

pub fn counting_nodes(g: &Gfa) -> count_node{
    let mut nc: HashMap<String, u32> = HashMap::new();
    for x in &g.nodes{
        nc.insert(x.0.clone(), 0);
    }

    for x in &g.paths{

        let v: HashSet<_> = x.nodes.iter().cloned().collect();
        for y in v{
            *nc.get_mut(&y).unwrap() += 1;
        }
    }
    count_node{ncount: nc}
}

