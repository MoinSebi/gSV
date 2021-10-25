use std::collections::HashMap;


/// Convert index in the graph to positional information
/// Index based - not node based
/// [10,30,31,32,45]
pub fn graph2pos(graph: & gfaR_wrapper::NGfa) -> HashMap<String, Vec<usize>>{
    let mut result_hm: HashMap<String, Vec<usize>> = HashMap::new();
    for x in graph.paths.iter(){
        let mut vec_pos: Vec<usize> = Vec::new();
        let mut position: usize = 0;
        for y in x.nodes.iter(){
            position += graph.nodes.get(y).unwrap().seq.len();
            vec_pos.push(position);
        }
        result_hm.insert(x.name.clone(), vec_pos);
    }
    result_hm
}