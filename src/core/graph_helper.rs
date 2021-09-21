use std::collections::HashMap;



pub fn graph2pos(graph: & gfaR_wrapper::NGfa) -> HashMap<String, Vec<usize>>{
    let mut hm: HashMap<String, Vec<usize>> = HashMap::new();
    for x in graph.paths.iter(){
        let mut vec_pos: Vec<usize> = Vec::new();
        let mut posi: usize = 0;
        for y in x.nodes.iter(){
            posi += graph.nodes.get(y).unwrap().seq.len();
            vec_pos.push(posi);
        }
        hm.insert(x.name.clone(), vec_pos);
    }
    hm
}