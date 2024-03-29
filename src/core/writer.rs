use crate::core::core::Bubble;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Write, BufWriter};
use crate::panSV::panSV_core::{BubbleWrapper};
use crate::core::helper::{bool2string_dir, hashset2string};

/// Write bubbles with unique id
/// Read doc/bubble.stats
/// Difference to "bubble_naming_old"
/// - no complex naming (no recursion)
/// - additional file for "parent ids"
pub fn bubble_naming_new(hm1: & HashMap<u32, Bubble>, out: &str){
    let f = File::create([out, "bubble", "stats"].join(".")).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    write!(f, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
           "bubbleID",
           "#Nestedness",
           "#subBubbles",
           "minLen",
           "maxLen",
           "meanLen",
           "#traversal",
           "#intervals",
           "Parents",
            "Anchor1",
            "Anchor2",
            "Ratio",
            "Small",
            "Type", ).expect("Can not write stats file");
    for (_k,v) in hm1.iter(){
        let (max, min ,mean) = v.traversal_stats();
        write!(f, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\tCL:{}\n", v.id, v.nestedness, v.children.len(), min, max, mean, v.traversals.len(), v.number_interval(), hashset2string(&v.parents, ","), v.start, v.end, v.ratio, v.small, v.category, v.core).expect("Not able to write bubble stats");
    }
}


/// Naming bubble parent-child relationship
///
/// Additional file nedded for new bubble naming
pub fn bubble_parent_structure(hm1: & HashMap<u32, Bubble>, out: &str){
    let f = File::create([out, "bubble", "txt"].join(".")).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    write!(f, "bubble_id\tchildren_id\tparents_id\n").expect("Not able to write bubble nestedness file");
    for (_k,v) in hm1.iter(){
        write!(f, "{}\t{:?}\t{:?}\n", v.id, v.children, v.parents).expect("Not able to write bubble nestedness file");
    }
}



/// Old naming
pub fn bubble_naming_old(hm1: & HashMap<u32, Bubble>, naming: & mut HashMap<u32, Vec<u32>>,  out: &str, maxcore: &u32){
    naming_wrapper(hm1, maxcore, naming);
    let f = File::create([out, "stats"].join(".")).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    write!(f, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
           "bubbleID",
           "coreNumber",
           "subBubbles",
           "minLen",
           "maxLen",
           "meanLen",
           "#traversal",
           "#intervals").expect("Can not write stats file");
    for (_k, v) in hm1{
        let j:Vec<String> = naming.get(&v.id).unwrap().iter().map(|i| i.to_string()).collect();
        let (max, min ,mean) = v.traversal_stats();
        write!(f, "{}\t{}\t{}\t{}\t{}\t{:.2}\t{}\t{}\n",
               j.join("."),
               v.core,
               v.children.len(),
               min,
               max,
               mean,
               v.traversals.len(),
               v.number_interval(),

        ).expect("Can not write file");
    }
}

/// Naming bubbles pre Rust
/// Only needed for bubble_naming_old
/// Requirement: Bubbles can only have one parent
/// But: This is not the case in panSV
/// Changed stuff: No multiple names anymore
pub fn bubble_names(hm1: & HashMap<u32, Bubble>, s: &Vec<u32>, nodeid: &u32, maxcore: &u32, number: &u32, naming: & mut HashMap<u32, Vec<u32>>){
    let bubble = hm1.get (& nodeid).unwrap();
    let core = bubble.core;
    let mut h = s.clone();
    // Auffüllen
    for _x in s.len() as u32..*maxcore-core{
        h.push(0);
    }
    h.push(number.clone());

    let mut h2 = h.clone();
    for _x in 2..core {
        h2.push(0 as u32);
    }
    naming.insert(bubble.id.clone(), h2.clone());

    if bubble.children.len() > 0 {
        let mut hm_test: HashMap<u32, u32> = HashMap::new();
        for x in bubble.children.iter(){
            if hm_test.contains_key(&hm1.get(x).unwrap().core){
                let u = hm_test.get(&hm1.get(x).unwrap().core).unwrap();
                bubble_names(hm1, &h, x, maxcore, &u, naming);
                *hm_test.get_mut(&hm1.get(x).unwrap().core).unwrap() += 1;

            } else {
                bubble_names(hm1, &h, x, maxcore, &1, naming);
                hm_test.insert(hm1.get(x).unwrap().core.clone(), 2);
            }
        }
    }
}


/// This is a wrapper for bubble_naming_old
///
/// For parents -> each core has single list of numbers
pub fn naming_wrapper(hm1: & HashMap<u32, Bubble>, maxcore: &u32, naming: &mut HashMap<u32, Vec<u32>>){

    let mut _it = 1;
    let mut h: HashMap<u32, u32> = HashMap::new();
    for (k,v) in hm1.iter(){
        if v.parents.len() == 0{
            if h.contains_key(&v.core){
                *h.get_mut(&v.core).unwrap() += 1;
            } else {
                h.insert(v.core, 1);
            }
            let j: Vec<u32> = vec![];

            bubble_names(hm1, &j, k, maxcore, &h.get(&v.core).unwrap(), naming);
            _it += 0;
        }
    }
}

/// Writing bed file
/// Accession - FROM - TO - BUBBLE ID - BUBBLE CORE - TRAVERSAL
/// Iterate over id2interval bubble_wrapper
pub fn writing_bed(r: &BubbleWrapper, index2: & HashMap<String, Vec<usize>>, out: &str){

    let f = File::create([out, "bed"].join(".")).expect("Unable to create file");
    let mut f = BufWriter::new(f);

    for (_k,v) in r.id2interval.iter() {
        let from_id: usize = index2.get(&v.acc).unwrap()[v.from as usize];
        let mut to_id:usize = index2.get(&v.acc).unwrap()[v.to as usize-1];

        if v.to == v.from+1{
            to_id = from_id.clone();
        }
        let bub = r.id2bubble.get(r.id2id.get(&(v.from, v.to, &v.acc)).unwrap()).unwrap();
        let (max, min ,_mean) = bub.traversal_stats();
        write!(f, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
               v.clone().acc,
               from_id,
               to_id,
               bub.id,
               bub.core,
                bub.category,
                bub.small ,
                max,
                min,).expect("Not able to write to file");
    }
}


/// Write bed file
/// Documentation found here doc.md
pub fn writing_bed_traversals(h: &BubbleWrapper, index2: & HashMap<String, Vec<usize>>, out: &str){
    let f = File::create([out, "traversal", "bed"].join(".")).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    for x in h.id2bubble.iter(){
        for y in x.1.traversals.iter(){
            for x1 in y.1.pos.iter(){

                let k = h.id2interval.get(&x1).unwrap();
                let from_id: usize = index2.get(&k.acc).unwrap()[k.from as usize];
                let mut to_id:usize = index2.get(&k.acc).unwrap()[k.to as usize-1];
                if k.to == k.from+1{
                    to_id = from_id.clone();
                }
                let bub = h.id2bubble.get(h.id2id.get(&(k.from, k.to, &k.acc)).unwrap()).unwrap();
                write!(f, "{}\t{}\t{}\t{}\t{}\t{}\n", k.acc, from_id, to_id, bub.id, bub.core, y.1.id).expect("Can't write traversal file");
            }


        }
    }
}



/// Writing traversal file
/// Printing:
/// traversal Len bubble
pub fn writing_traversals(h: &BubbleWrapper, out: &str){
    let f = File::create([out, "traversal", "txt"].join(".")).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    for x in h.id2bubble.iter(){

        for y in x.1.traversals.iter(){
            let mut o: Vec<String> = Vec::new();
            for x1 in y.0.iter(){
                let j: String =  x.0.to_string() + &bool2string_dir(x1.1);
                o.push(j);

            }

            //write!(f, "{}\t{}\t{}\n", o.join(","), y.1.length, vec2string(&naming.hm.get(&x.1.id).unwrap(), ".")).expect("Can't write traversal file");
            write!(f, "{}\t{}\t{}\n", o.join(","), y.1.length, y.1.id).expect("Can't write traversal file");

        }
    }
}


/// Writing traversal file
/// Printing:
/// traversal Len bubble
pub fn writing_uniques_bed(h: &BubbleWrapper, index2: & HashMap<String, Vec<usize>>, out: &str, size: usize){
    let f = File::create([out, "traversal", "unique", "bed"].join(".")).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    for x in h.id2bubble.iter(){

        for y in x.1.traversals.iter(){
            let k = h.id2interval.get(&y.1.pos[0]).unwrap();
            let from_id: usize = index2.get(&k.acc).unwrap()[k.from as usize];
            let mut to_id:usize = index2.get(&k.acc).unwrap()[k.to as usize-1];
            if k.to == k.from+1{
                to_id = from_id.clone();
            }

            if to_id - from_id > size{
                write!(f, "{}\t{}\t{}\n", k.acc, from_id, to_id).expect("Can't write traversal file");
            }

            //write!(f, "{}\t{}\t{}\n", o.join(","), y.1.length, vec2string(&naming.hm.get(&x.1.id).unwrap(), ".")).expect("Can't write traversal file");

        }
    }
}



/// Writing traversal file
/// Printing:
/// bubble_id traversalid nodes, nodes
pub fn writing_uniques_bed_stats(h: &BubbleWrapper, index2: & HashMap<String, Vec<usize>>, out: &str, size: usize){
    let f = File::create([out, "traversal", "unique", "bubble", "bed"].join(".")).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    for x in h.id2bubble.iter(){

        for y in x.1.traversals.iter(){
            let k = h.id2interval.get(&y.1.pos[0]).unwrap();
            let from_id: usize = index2.get(&k.acc).unwrap()[k.from as usize];
            let mut to_id:usize = index2.get(&k.acc).unwrap()[k.to as usize-1];
            if k.to == k.from+1{
                to_id = from_id.clone();
            }
            let mut o: Vec<String> = Vec::new();
            for x1 in y.0.iter(){
                let j: String =  x1.0.to_string() + &bool2string_dir(x1.1);
                o.push(j);

            }

            if to_id - from_id > size{
                write!(f, "{}\t{}\t{}\n", x.1.id, y.1.id, o.join(",")).expect("Can't write traversal file");
            }

            //write!(f, "{}\t{}\t{}\n", o.join(","), y.1.length, vec2string(&naming.hm.get(&x.1.id).unwrap(), ".")).expect("Can't write traversal file");

        }
    }
}