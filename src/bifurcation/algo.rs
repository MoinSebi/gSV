use gfaR_wrapper::{NGfa};
use std::collections::{HashSet, HashMap};
use bifurcation::from_gfaR::iterate_test;
use log::{info};
use crate::{sort_trav};
use crate::panSV::panSV_core::{PanSVpos};


/// Bifurcation wrapper
/// TODO
/// Remove some iterations and memory problems
pub fn bifurcation_wrapper(graph: &NGfa, threads: &usize) -> HashMap<String, Vec<PanSVpos>> {
    info!("Running bifurcation analysis");

    let result = iterate_test(graph, threads.clone()) ;

    // Output: Vec((String, String), {(usize,usize) -> Vec<(usize, usize)}, Option
    let mut result_temp = HashMap::new();

    // Input -> String -> Start, Stop
    info!("Sorting and merging results");
    for x in result.iter(){
        let mut v1 = Vec::new();
        let mut v2 = Vec::new();
        for (key, value) in x.1.0.iter(){
            for val in value.iter(){
                v1.push((key.0.clone(), val.0.clone()));
                v2.push((key.1.clone(), val.1.clone()));
            }

        }
        result_temp.entry(&x.0.0).or_insert(v1.clone()).extend(v1.clone());
        result_temp.entry(&x.0.1).or_insert(v2.clone()).extend(v2.clone());

    }

    // Vector -> HashSet(This can be done faster)
    let mut result_merge = HashMap::new();
    for (key, val) in result_temp.iter(){
        let mut j = HashSet::new();
        for x in val.iter(){
            j.insert((x.0 as u32,x.1 as u32, 0));
        }
        //debug!("panPos size {}", j.len());
        //debug!("panPos size {:?}", j);
        result_merge.insert(key.to_owned().clone(), j);
    }

    // Pan_Pos -> (string, start, stop);
    let mut result_panpos = HashMap::new();
    for (key, val) in result_merge.iter(){
        let mut j = Vec::new();
        for x in val.iter(){
            j.push(PanSVpos{start: x.0, end: x.1, core: x.2})
        }
        for x in j.iter(){
            if x.end < x.start{
                print!("help");
                print!("{:?}", x);
            }
        }
        result_panpos.insert(key.to_owned().clone(), j);
    }
    //print!("{:?}", result_panpos);

    let result_panpos_final = sort_trav(result_panpos);
    result_panpos_final

}


#[cfg(test)]
mod bifurcation_test {
    use gfaR_wrapper::NGfa;
    use log::trace;
    use crate::bifurcation::algo::{bifurcation_wrapper};
    use crate::{algo_panSV, CountNode, create_bubbles, graph2pos, indel_detection};
    use crate::panSV::algo::sort_trav;

    fn loadthegraph() -> NGfa{
        let mut graph = NGfa::new();
        graph.from_graph("example_data/testGraph.gfa");
        return graph
    }

    #[test]
    fn test_run() {
        let graph = loadthegraph();
        let h = bifurcation_wrapper(&graph, &2);
        let g2p = graph2pos(&graph);
        let j = sort_trav(h);
        let o = create_bubbles(& j, & graph.paths, &g2p);
        trace!("{:?}", o.id2bubble);

    }

    #[test]
    fn test_normal(){
        let graph = loadthegraph();
        let mut counts: CountNode = CountNode::new();
        counts.counting_graph(&graph);
        let (o,_m) = algo_panSV(&graph.paths, &counts);
        let h = graph2pos(&graph);
        let mut gg = create_bubbles(&o, &graph.paths, &h);
        let interval_numb = gg.id2interval.len() as u32;
        indel_detection(& mut gg, &graph.paths, interval_numb);
        trace!("{:?}", gg.id2bubble);

    }
}
