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
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;
use crate::panSV::panSV_core::PanSVpos;

/// Get all path pairs of a graph
pub fn get_all_pairs(graph: &NGfa) -> Vec<(&NPath, &NPath)> {
    let mut pairs: Vec<(&NPath, &NPath)> = Vec::new();
    for (i1, path1) in graph.paths.iter().enumerate(){
        for path2 in graph.paths[i1+1..].iter(){
            // Optional for checking
            // println!("{} {}", path1.name, path2.name);
            pairs.push((path1, path2));
        }
    }
    pairs
}

/// Get shared values between two paths (only the nodes)
/// Only nodes and ignoring direction
pub fn get_shared(test: &NPath, test2: &NPath){
    let u = 32;
    let k: HashSet<u32> = HashSet::from_iter(test.nodes.clone());
    let k2: HashSet<u32> = HashSet::from_iter(test2.nodes.clone());
    let g: HashSet<&u32> = k.intersection(&k2).collect();
}


/// Get shared nodes between two paths (with direction correct)
/// Complexity is O(N²)
pub fn get_shared_direction(test: &NPath, test2: &NPath){
    let mut g: HashMap<(u32, bool), [Vec<usize>; 2]> = HashMap::new();
    let iter: HashSet<(&u32, &bool)> = HashSet::from_iter(test.nodes.iter().zip(test.dir.iter()));
    let iter2: HashSet<(&u32, &bool)> = HashSet::from_iter(test2.nodes.iter().zip(test2.dir.iter()));

    let g: HashSet<&(&u32, &bool)> = iter.intersection(&iter2).collect();
    println!("The length of shared nodes is {}", g.len());
    println!("Shared nodes {:?}", g);
}

#[cfg(test)]
mod tests {
    use crate::core::helper::{bool2string_dir, vec2string, hashset2string};
    use std::collections::HashSet;
    use gfaR_wrapper::NGfa;
    use crate::bifurcation::algo::{get_all_pairs, get_shared, get_shared_direction};

    #[test]
    fn pairs(){
        let graph = NGfa::new();
        let mut graph: NGfa = NGfa::new();
        graph.from_graph("example_data/testGraph.gfa");
        assert_eq!(get_all_pairs(&graph).len(), 6)

    }

    #[test]
    fn get_shared_all(){
        let graph = NGfa::new();
        let mut graph: NGfa = NGfa::new();
        graph.from_graph("example_data/testGraph.gfa");
        let h = get_all_pairs(&graph);
        get_shared_direction(h[0].0, h[0].1);

    }
}
