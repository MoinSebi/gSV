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
use gfaR_wrapper::{NGfa, GraphWrapper, NPath};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;


/// Get shared values between two paths (only the nodes)
pub fn get_shared(test: &NPath, test2: &NPath){
    let u = 32;
    let k: HashSet<u32> = HashSet::from_iter(test.nodes.clone());
    let k2: HashSet<u32> = HashSet::from_iter(test2.nodes.clone());
    let g: HashSet<&u32> = k.intersection(&k2).collect();

}