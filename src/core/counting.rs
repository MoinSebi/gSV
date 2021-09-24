use std::collections::{HashMap, HashSet};
use gfaR_wrapper::NGfa;




pub struct CountNode {
    pub ncount: HashMap<u32, u32>,
}

impl CountNode{
    pub fn new() -> Self{
        let ncount: HashMap<u32, u32> = HashMap::new();
        Self{
            ncount,
        }
    }

    pub fn all_path(& mut self, g: &NGfa){
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
        self.ncount  = nc;
    }

    pub fn from_genomes(& mut self, g: &NGfa, g2: &gfaR_wrapper::GraphWrapper){
        let mut nc: HashMap<u32, u32> = HashMap::new();
        for x in &g.nodes{
            nc.insert(x.0.clone(), 0);
        }
        for (_k,v) in g2.genomes.iter(){
            let mut hh: HashSet<u32> = HashSet::new();
            for y in v.iter(){
                let v: HashSet<_> = y.nodes.iter().cloned().collect();
                hh.extend(&v);
            }
            for y in hh.iter(){
                *nc.get_mut(&y).unwrap() += 1;
            }
        }
        self.ncount = nc;
    }

}


