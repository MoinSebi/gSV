use std::collections::{HashMap};
use crate::core::counting::{CountNode};
use crate::panSV::panSV_core::{PanSVpos, TmpPos, BubbleWrapper, OldNaming};
use crate::core::core::{Posindex, Bubble, Traversal};
use related_intervals::{make_nested, Network};
use std::fs::File;
use std::io::{Write, BufWriter};
use gfaR_wrapper::NPath;


/// New starters, always from 0!!! Ending strategy
pub fn new_starter(j: usize, s: &String) -> Vec<TmpPos> {
    let mut opentrans:  Vec<TmpPos> = Vec::new();
    for x in 2..j+1{
        opentrans.push(TmpPos {acc: s.to_owned(), start: 0, core: x as u32, border: true});
    }
    opentrans

}


#[allow(non_snake_case)]
/// PanSV algorithm
///
/// Input: paths, counts
/// Output: HM(String -> Vec<pos>)
/// panSVpos: index, index, core
///
pub fn algo_panSV(paths: & Vec<NPath>, counts: &CountNode) -> (HashMap<String, Vec<PanSVpos>>, HashMap<String, usize>) {

    let mut lastcore: u32;
    #[allow(non_snake_case)]
    let mut result_panSV: HashMap<String, Vec<PanSVpos>> = HashMap::new();

    let mut max_index: HashMap<String, usize> = HashMap::new();

    // We create String -> (start, stop, core)
    for x in paths{
        let ki: Vec<_> = Vec::new();
        result_panSV.insert(x.name.to_owned().clone(), ki);
    }

    // Iterate over each path
    for x in paths {
        // Need max index later
        max_index.insert(x.name.clone(), x.nodes.len()-1);
        lastcore = 1;

        // All "open" intervals
        let mut interval_open:  Vec<TmpPos> = new_starter(paths.len(), &x.name);

        println!("Path name: {}", x.name);
        // Iterate over all nodes
        for (index, node) in x.nodes.iter().enumerate() {
            // if core is smaller than before -> open new bubble
            if counts.ncount[node] < lastcore {
                interval_open.push(TmpPos { acc: x.name.clone(), start: (index - 1) as u32, core: lastcore, border: false});
                lastcore = counts.ncount[node];
            }
            // If bigger -> close bubble
            else if counts.ncount[node] > lastcore {
                lastcore = counts.ncount[node];

                // There is no bubble opened with this core level
                let mut trig = false;

                // List which open trans are removed later
                let mut remove_list: Vec<usize> = Vec::new();


                // We iterate over all open bubbles
                for (index_open, o_trans) in interval_open.iter().enumerate() {
                    // Check if we find the same core level
                    if o_trans.core == counts.ncount[node] {
                        trig = true;
                    }


                    // If one open_interval has smaller (or same) core level -> close
                    if o_trans.core <= counts.ncount[node] {
                        // why this?
                        if index != 0 {
                            result_panSV.get_mut(&o_trans.acc).unwrap().push(PanSVpos {start: o_trans.start, end: index as u32, core: o_trans.core, border: o_trans.border});

                        }
                        remove_list.push(index_open);
                    }
                }
                // Remove stuff from the interval_open list
                for (index_r, index_remove) in remove_list.iter().enumerate() {
                    interval_open.remove(*index_remove - index_r);
                }

                // If there is not a open interval which has the same core level -> this still exists
                if !trig {
                    //println!("BIG HIT");
                    result_panSV.get_mut(&x.name).unwrap().push(PanSVpos {start: interval_open[interval_open.len() - 1].start, end: index as u32, core: lastcore, border: false});
                }
            }

        }
        // This the other end - its one longer than the rest (identifier)
        for otrans in interval_open.iter(){
            if (otrans.start != 0) & (otrans.border != true){
                result_panSV.get_mut(&otrans.acc).unwrap().push(PanSVpos {start: otrans.start, end: (x.nodes.len()-1) as u32, core: otrans.core, border: true});

            }
        }

    }
    let result_result = sort_trav(clean_borders(result_panSV, &max_index));
    //println!("{:?}", result_result);

(result_result, max_index)
}



/// Remove non unique borders
///
///
///
///
pub fn clean_borders(old_result: HashMap<String, Vec<PanSVpos>>, max_index: &HashMap<String, usize>) -> HashMap<String, Vec<PanSVpos>> {
    // New result
    let mut new_result:  HashMap<String, Vec<PanSVpos>> = HashMap::new();

    // Iterate over all entries
    for (k,v) in old_result.iter(){
        // new vector, rep
        let mut new_vec:  Vec<PanSVpos> = Vec::new();
        let mut rep_starts: Vec<PanSVpos> = Vec::new();
        let mut rep_ends: Vec<PanSVpos> = Vec::new();
        // Alle "normalen" traversals adden
        for x in v{
            if x.end == *max_index.get(k).unwrap() as u32{
                rep_ends.push(x.clone());
            }
            else if (x.start > 0) & (x.end < *max_index.get(k).unwrap() as u32){
                new_vec.push(x.clone());
            } else{
                if x.end != 0 {
                    rep_starts.push(x.clone());
                }
            }

        }

        // For starts, hold all different entry points (+ if same points, hold biggest core)

        let mut starts: HashMap<u32, Vec<(u32, bool)>> = HashMap::new();
        for x in rep_starts.iter(){
            if starts.contains_key(&x.end){
                starts.get_mut(&x.end).unwrap().push((x.core, x.border));
            }
            else {
                starts.insert(x.end, vec![(x.core, x.border)]);
            }

        }
        for (k22,v22) in starts.iter(){
            let mut max1: (u32, bool) =  (0, false);
            for (x,y) in  v22.iter(){
                if x > &max1.0{
                    max1 = (x.clone(), y.clone());
                }
            }
            new_vec.insert(0, PanSVpos {start: 0, end: k22.clone(), core: max1.0.clone(), border: max1.1.clone()});

        }




        // For ends, hold all different ending points (+ if same points, hold smallest core)
        // Maybe redundant
        let mut ends: HashMap<u32, Vec<(u32, bool)>> = HashMap::new();
        for x in rep_ends.iter(){
            if ends.contains_key(&x.start){
                ends.get_mut(&x.start).unwrap().push((x.core, x.border));
            }
            else {
                ends.insert(x.start, vec![(x.core, x.border)]);
            }

        }

        for (k22,v22) in ends.iter(){
            let mut max1 = (0, false);
            for (x,y) in  v22.iter(){
                if x > &max1.0{
                    max1 = (x.clone(), y.clone());
                }
            }
            new_vec.insert(0, PanSVpos {start: k22.clone(), end: *max_index.get(k).unwrap() as u32, core: max1.0, border: max1.1})
        }




        // we add all to new rsult!
        new_result.insert(k.to_owned().clone(), new_vec);
    }
    new_result

}

/// Sorting vector in hashmaps
///
/// smallest a into biggest b
pub fn sort_trav(result:  HashMap<String, Vec<PanSVpos>>) -> HashMap<String, Vec<PanSVpos>>{

    let mut new_result: HashMap<String, Vec<PanSVpos>> = HashMap::new();


    for (k, v) in result.iter(){
        let mut j = Vec::new();
        for x in v.iter(){
            j.push(x.clone());
        }
        j.sort_by(|a, b| (a.start.cmp(&b.start).then(b.end.cmp(&a.end))));
        new_result.insert(k.to_owned().clone(), j) ;
        //v.sort_by(|a, b| a.partial_cmp(b).unwrap());

    }
    new_result
}






/// We make bubbles *_* lets go!
///
///
pub fn create_bubbles<'a, 'b>(inp: &'a HashMap<String, Vec<PanSVpos>>, p: &'a   Vec<NPath>, ghm: &'b HashMap<String, Vec<usize>>) -> BubbleWrapper<'a>{

    let mut result: BubbleWrapper = BubbleWrapper::new();

    let mut tcount = 0;
    let mut bcount = 0;

    for x in p{
        for pos in inp[&x.name].iter(){

            let newbub = (& x.nodes[pos.start as usize], & x.nodes[pos.end as usize]);
            let len_trav: usize  = ghm.get(&x.name).unwrap()[pos.end as usize-1] -  ghm.get(&x.name).unwrap()[pos.start as usize];

            let tt = Traversal{length: len_trav as u32, pos: vec![tcount]};

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
                let k: Vec<u32> = x.nodes[(pos.start+1) as usize..pos.end as usize].iter().cloned().collect();
                let k2: Vec<bool> = x.dir[(pos.start+1) as usize..pos.end as usize].iter().cloned().collect();

                let mut k10: Vec<(u32, bool)> = Vec::new();
                for x in 0..k.len(){
                    k10.push((k[x], k2[x]));
                }
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

                    result.id2bubble.get_mut(temp_bcount).unwrap().traversals.insert(k10,tt);
                    //pV.id2bubble.get_mut(temp_bcount).unwrap().traversals.insert(k,tt);

                }

                result.id2interval.insert(tcount, Posindex {from: pos.start.clone(), to: pos.end.clone(), acc: x.name.clone(), border: pos.border.clone()});

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
                let k: Vec<u32> = x.nodes[(pos.start+1) as usize..pos.end as usize].iter().cloned().collect();
                let k2: Vec<bool> = x.dir[(pos.start+1) as usize..pos.end as usize].iter().cloned().collect();

                let mut k10: Vec<(u32, bool)> = Vec::new();
                for x in 0..k.len(){
                    k10.push((k[x], k2[x]));
                }




                result.anchor2bubble.insert(newbub, bcount);
                result.id2bubble.insert(bcount, Bubble::new(pos.core.clone(), x.nodes[pos.start as usize].clone(), x.nodes[pos.end as usize].clone(),
                                                            tcount, bcount, tt, k10, pos.border));
                result.id2interval.insert(tcount, Posindex {from: pos.start.clone(), to: pos.end.clone(), acc: x.name.clone(), border: pos.border.clone()});
                result.anchor2interval.insert((&pos.start, &pos.end, &x.name), tcount);
                result.id2id.insert((pos.start.clone(), pos.end.clone(), &x.name), bcount);





                bcount += 1;
            }
            tcount += 1;


        }

    }
    // Connect bubbles
    connect_bubbles_wrapper(inp, & mut result);

    result

}

/// Indel detection
///
/// Iterate over nodes in path
/// If two nodes after each othera are borders of bubbles
/// Add traversal to bubble
pub fn indel_detection<'a>(r: & mut BubbleWrapper<'a>, paths: &'a Vec<NPath>, last_id: u32){
    let mut ll = last_id.clone() + 1;

    for path in paths.iter(){
        for x in 0..path.nodes.len()-1{
            let ind = (&path.nodes[x], &path.nodes[x+1]);
            if r.anchor2bubble.contains_key(&ind){

                let bub =  r.id2bubble.get_mut(r.anchor2bubble.get(&ind).unwrap()).unwrap();
                //if ! bub.acc.contains(& path.name) {
                if ! bub.border {
                    let k: Vec<(u32, bool)> = vec![];
                    let jo: Traversal = Traversal::new(ll, 0);
                    r.id2interval.insert(ll, Posindex { from: (x as u32), to: ((x + 1) as u32), acc: path.name.clone(), border: false });
                    bub.border = false;
                    r.id2id.insert(((x as u32), ((x + 1) as u32), &path.name), bub.id.clone());
                    if bub.traversals.contains_key(&k) {
                        bub.traversals.get_mut(&k).unwrap().pos.push(ll);
                    } else {
                        bub.traversals.insert(k.clone(), jo);
                        bub.traversals.get_mut(&k).unwrap().pos.push(ll);
                    }
                    ll += 1;
                }
                //}

            }
        }
    }
}

pub fn writing_bed(r: &BubbleWrapper, index2: & HashMap<String, Vec<usize>>, max_index: &HashMap<String, usize>, out: &str){

    let f = File::create([out, "bed"].join(".")).expect("Unable to create file");
    let mut f = BufWriter::new(f);

    for (_k,v) in r.id2interval.iter() {
        let mut from_id: usize = index2.get(&v.acc).unwrap()[v.from as usize];
        let mut to_id:usize = index2.get(&v.acc).unwrap()[v.to as usize-1];

        if (v.border) & (v.from == 0)  {
            from_id = 0;
        } else if (v.border) & (v.to == *max_index.get(&v.acc).unwrap() as u32 ){
            to_id = index2.get(&v.acc).unwrap()[v.to as usize];
        } else if v.to == v.from+1{
            to_id = from_id.clone();
        }
        let bub = r.id2bubble.get(r.id2id.get(&(v.from, v.to, &v.acc)).unwrap()).unwrap();

        write!(f, "{}\t{}\t{}\t{}\t{}\n",
               v.clone().acc,
               from_id,
               to_id,
                bub.id,
               bub.core).expect("Not able to write to file");
               //f.write_all("{} {} {} {}", v.acc, v.from, v.to, index.get(&v).unwrap());
    }
}

pub fn writing_traversals(h: &BubbleWrapper, naming: &OldNaming, out: &str){
    let f = File::create([out, "traversal", "txt"].join(".")).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    for x in h.id2bubble.iter(){

        for y in x.1.traversals.iter(){
            let mut o: Vec<String> = Vec::new();
            for x in y.0.iter(){
                let j: String =  x.0.to_string() + &bool2string_dir(x.1);
                o.push(j);

            }

            write!(f, "{}\t{}\t{}\n", o.join(","), y.1.length, vec2string(&naming.hm.get(&x.1.id).unwrap())).expect("Can't write traversal file");
        }
    }
}

pub fn bool2string_dir(b: bool) -> String{
    if b{
        return "+".to_string();

    } else {
        return "-".to_string();
    }

}

pub fn vec2string(input: &Vec<u32>) -> String{
    let j:Vec<String> = input.iter().map(|i| i.to_string()).collect();
    j.join(".")


}

pub fn connect_bubbles_wrapper(hm: &HashMap<String, Vec<PanSVpos>>, result: &  mut BubbleWrapper){
    println!("Connect bubbles");
    let mut network: HashMap<(u32, u32), Network>;
    for (k,v) in hm.iter(){
        println!("Connecting bubbles from {}", k);
        let mut jo: Vec<(u32, u32)> = Vec::new();
        for x in v.iter() {
            jo.push((x.start.clone(), x.end.clone()));
        }
        network = related_intervals::create_network_hashmap(&jo);


        make_nested(&jo, & mut network);

        connect_bubbles(&network,  result, &k);

    }
}


pub fn connect_bubbles(hm: &HashMap<(u32, u32), Network>, result: & mut BubbleWrapper, s: &String){



    for (k,v) in hm.iter(){


        let index = result.id2id.get(&(k.0, k.1, s)).unwrap();
        let mut ii: Vec<&u32> = Vec::new();
        for x in v.parent.iter(){
            ii.push(result.id2id.get(&(x.0, x.1, s)).unwrap());
        }



        for x in ii.iter(){
            result.id2bubble.get_mut(x).unwrap().children.insert(index.clone());
            result.id2bubble.get_mut(index).unwrap().parents.insert(x.clone().clone());
        }

    }

}
