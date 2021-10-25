use std::collections::{HashMap, HashSet};
use gfaR_wrapper::NGfa;



/// Dummy struct for counting number of traversals
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

    /// Counting for each path
    pub fn counting_graph(& mut self, graph: &NGfa){
        let mut count: HashMap<u32, u32> = HashMap::new();
        for x in &graph.nodes{
            count.insert(x.0.clone(), 0);
        }

        for x in &graph.paths{

            let v: HashSet<_> = x.nodes.iter().cloned().collect();
            for y in v{
                *count.get_mut(&y).unwrap() += 1;
            }
        }
        self.ncount  = count;
    }

    /// Counting for each genome
    /// This includes the graph_wrapper
    pub fn counting_wrapper(& mut self, graph: &NGfa, graph_wrapper: &gfaR_wrapper::GraphWrapper){
        let mut count: HashMap<u32, u32> = HashMap::new();
        for x in &graph.nodes{
            count.insert(x.0.clone(), 0);
        }
        for (_k,v) in graph_wrapper.genomes.iter(){
            let mut combined_nodes: HashSet<u32> = HashSet::new();
            for y in v.iter(){
                let v: HashSet<_> = y.nodes.iter().cloned().collect();
                combined_nodes.extend(&v);
            }
            for y in combined_nodes.iter(){
                *count.get_mut(&y).unwrap() += 1;
            }
        }
        self.ncount = count;
    }

}






