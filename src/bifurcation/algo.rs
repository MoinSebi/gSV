// pub fn find_parents(h: &HashMap<&String, Vec<(u32, u32, u32)>>, hu: &mut HashMap< u32, Bubble>, thekey: &HashMap<u32, u32>){
//     let mut opens: Vec<usize> = Vec::new();
//     println!("{:?}", thekey);
//     // iterating over the the vector acc -> (start stop)
//     for i in h.iter(){
//        opens = Vec::new();
//         for (ii, x) in i.iter().enumerate(){
//             if opens.len() == 0{
//                 opens.push(ii)
//             } else {
//                 // this is why
//                 let mut vecc = Vec::new();
//                 for (indi, oi) in opens.iter().rev().enumerate(){
//                     if i[*oi].1 < x.0{
//                         vecc.push(indi);
//                         if indi == opens.len() -1{
//                         } else {
//                             let k = hu[&((thekey[&i[oi-1].2]))].id.clone();
//                             println!("dasdad {:?}",k);
//                             hu.get_mut(&(thekey[&i[oi-1].2])).unwrap().addChild(k);
//
//                         }
//                     }
//                 }
//                 opens.push(ii);
//                 println!("Weggle {:?}", vecc);
//             }
//         }
//         println!("{}", opens.len());
//         println!("{:?}", opens);
//         if opens.len() != 0{
//             for end in (1..opens.len()).rev(){
//                 println!("{:?}", thekey);
//                 let k = hu[&((thekey[&i[end-1].2]))].id.clone();
//                 let k2 = hu[&((thekey[&i[end].2]))].id.clone();
//                 println!("{:?}",k);
//                 hu.get_mut(&(thekey[&i[end].2])).unwrap().addChild(k);
//                 hu.get_mut(&(thekey[&i[end-1].2])).unwrap().addPar(k2);
//                 //hu.get_mut(&(1 as u32)).unwrap().link(hu.get_mut(&(1 as u32)).unwrap());
//             }
//
//         }
//     }
// }
use gfaR_wrapper::{NGfa, NPath};
use std::collections::{HashSet, HashMap, BTreeSet};
use std::io;
use std::io::Write;
use std::iter::FromIterator;
use bifurcation::from_gfaR::iterate_test;
use crate::core::core::{Bubble, Posindex, Traversal};
use crate::graph2pos;
use crate::panSV::algo::connect_bubbles_wrapper;
use crate::panSV::panSV_core::{BubbleWrapper, PanSVpos};

///
///
pub fn test1(graph: &NGfa) -> HashMap<String, Vec<PanSVpos>> {
    eprintln!("HIT");

    let result = iterate_test(graph, 10) ;
    eprintln!("HIT");

    let mut rr = HashMap::new();
    eprintln!("HIT");

    for x in result.iter(){
        let mut v1 = Vec::new();
        let mut v2 = Vec::new();
        for (key, value) in x.1.0.iter(){
            for val in value.iter(){
                v1.push((key.0.clone(), val.0.clone()));
                v2.push((key.1.clone(), val.1.clone()));
            }

        }
        rr.entry(&x.0.0).or_insert(v1.clone()).extend(v1.clone());
        rr.entry(&x.0.1).or_insert(v2.clone()).extend(v2.clone());

    }
    eprintln!("HIT {}", rr.len());
    let mut h = HashMap::new();
    for (key, val) in rr.iter(){
        let mut j = HashSet::new();
        for x in val.iter(){
            j.insert((x.0 as u32,x.1 as u32, 0));
        }
        h.insert(key.to_owned().clone(), j);
    }

    let mut h2 = HashMap::new();
    for (key, val) in h.iter(){
        let mut j = Vec::new();
        for x in val.iter(){
            j.push(PanSVpos{start: x.0, end: x.1, core: x.2})
        }
        h2.insert(key.to_owned().clone(), j);
    }
    h2



}



/// We make bubbles *_* lets go!
///
///
pub fn create_bubbles2<'a, 'b>(inp: &'a HashMap<String, Vec<PanSVpos>>, p: &'a   Vec<NPath>, ghm: &'b HashMap<String, Vec<usize>>) -> BubbleWrapper<'a>{

    println!("\nCreate bubbles");
    let mut result: BubbleWrapper = BubbleWrapper::new();

    let mut tcount = 0;
    let mut bcount = 0;
    let mut trcount = 0;

    for (i,x) in p.iter().enumerate(){

        print!("({}/{}) {}\r", i+1, p.len(), x.name);
        io::stdout().flush().unwrap();


        for pos in inp[&x.name].iter(){

            let mut newbub = BTreeSet::new();
            newbub.insert(& x.nodes[pos.start as usize]);
            newbub.insert(& x.nodes[pos.end as usize]);
            let len_trav: usize  = ghm.get(&x.name).unwrap()[pos.end as usize-1] -  ghm.get(&x.name).unwrap()[pos.start as usize];

            let tt = Traversal{length: len_trav as u32, pos: vec![tcount], id: 0};
            let k: Vec<u32> = x.nodes[(pos.start+1) as usize..pos.end as usize].iter().cloned().collect();
            let k2: Vec<bool> = x.dir[(pos.start+1) as usize..pos.end as usize].iter().cloned().collect();

            let mut k10: Vec<(u32, bool)> = Vec::new();
            for x in 0..k.len(){
                k10.push((k[x], k2[x]));
            }

            /*
            If we have the bubble
            -> check if traversal in bubble
                yes -> add pos to traversal
                no -> create new traversal, add pos
            add all to the bubble
            */
            if result.anchor2bubble.contains_key(&newbub){

                // make traversal
                // Vec -> meta
                //println!("{:?}", k2);


                // This bubble we are looking at
                let temp_bcount = result.anchor2bubble.get(&newbub).unwrap();
                let bub = result.id2bubble.get_mut(temp_bcount).unwrap();
                result.anchor2interval.insert((&pos.start, &pos.end, &x.name), tcount);
                let bub_id = bub.id.clone();

                // Check if traversal already there
                if bub.traversals.contains_key(&k10){
                    result.id2bubble.get_mut(temp_bcount).unwrap().traversals.get_mut(&k10).unwrap().add_pos(tcount);

                    //pV.id2bubble.get_mut(temp_bcount).unwrap().traversals.get_mut(&k).unwrap().addPos(tcount);
                }
                else {

                    result.id2bubble.get_mut(temp_bcount).unwrap().traversals.insert(k10.clone(),tt);
                    result.id2bubble.get_mut(temp_bcount).unwrap().traversals.get_mut(&k10).unwrap().id = trcount;
                    trcount += 1;
                    //pV.id2bubble.get_mut(temp_bcount).unwrap().traversals.insert(k,tt);

                }

                result.id2id.insert((pos.start.clone(), pos.end.clone(), &x.name), bub_id);
                result.anchor2bubble.insert(newbub, bub_id);





                //pV.id2bubble.get_mut(& pV.Anchor2bubble[&newbub]).unwrap().addPos(tcount);

            } else {
                /*
                Create new bubble
                Create new traversal
                Create pos

                 please save how to make vector -> Btree
                 */
                // Make traversal


                result.anchor2bubble.insert(newbub, bcount);
                result.id2bubble.insert(bcount, Bubble::new(pos.core.clone(), x.nodes[pos.start as usize].clone(), x.nodes[pos.end as usize].clone(),
                                                            tcount, bcount, tt, k10.clone()));
                result.id2bubble.get_mut(&bcount).unwrap().traversals.get_mut(&k10).unwrap().id = trcount;

                result.anchor2interval.insert((&pos.start, &pos.end, &x.name), tcount);
                result.id2id.insert((pos.start.clone(), pos.end.clone(), &x.name), bcount);
                trcount += 1;




                bcount += 1;
            }
            result.id2interval.insert(tcount, Posindex {from: pos.start.clone(), to: pos.end.clone(), acc: x.name.clone()});

            tcount += 1;


        }

    }
    // Connect bubbles
    connect_bubbles_wrapper(inp, & mut result);

    result

}


#[cfg(test)]
mod tests {
    use crate::core::helper::{bool2string_dir, vec2string, hashset2string};
    use std::collections::HashSet;
    use gfaR_wrapper::NGfa;
    use crate::bifurcation::algo::{create_bubbles2, test1};
    use crate::{algo_panSV, CountNode, create_bubbles, graph2pos, indel_detection};
    use crate::panSV::algo::sort_trav;

    #[test]
    fn pairs() {
        let mut graph = NGfa::new();
        graph.from_graph("example_data/testGraph.gfa");
        let mut h = test1(&graph);
        let g2p = graph2pos(&graph);
        eprintln!("hello123124213a");
        let j = sort_trav(h);
        let o = create_bubbles2(& j, & graph.paths, &g2p);
        eprintln!("hello123124213a {:?}", o.id2bubble);

        let mut counts: CountNode = CountNode::new();
        counts.counting_graph(&graph);
        let (o,_m) = algo_panSV(&graph.paths, &counts);
        let h = graph2pos(&graph);
        let mut gg = create_bubbles(&o, &graph.paths, &h);
        let interval_numb = gg.id2interval.len() as u32;
        indel_detection(& mut gg, &graph.paths, interval_numb);

        eprintln!("hello123124213a {:?}", gg.id2bubble);


    }
}
