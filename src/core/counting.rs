use std::collections::{HashMap, HashSet};
use gfaR_wrapper::NGfa;




pub struct CountNode {
    pub ncount: HashMap<u32, u32>,
}


pub fn counting_nodes(g: &NGfa) -> CountNode {
    let mut nc: HashMap<u32, u32> = HashMap::new();
    for x in &g.nodes{
        nc.insert(x.0.clone(), 0);
    }

    for x in &g.paths{

        let v: HashSet<_> = x.nodes.iter().cloned().collect();
        for y in v{
            *nc.get_mut(&y).unwrap() += 1;
        }
    }
    CountNode {ncount: nc}
}

