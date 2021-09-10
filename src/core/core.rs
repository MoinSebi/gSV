use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;




#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Posindex {
    pub from:  u32,
    pub to:   u32,
    pub acc:  String,
    pub border: bool,
}

#[derive(Debug, Clone)]
pub struct Bubble {
    pub start: String,
    pub end: String,
    pub id: u32,
    pub children: HashSet<u32>,
    pub parents: HashSet<u32>,
    pub traversals: HashMap<Vec<String>, Traversal>,
    // this is kinda panSV specific
    pub core: u32

}


impl Bubble {
    /// Bubble constructor
    pub fn new(core: u32, start: String, end: String, first:u32, i: u32, trav: Traversal, s : Vec<String>) -> Self{
        let mut u: Vec<u32> = Vec::new();
        u.push(first);
        let u2: HashSet<u32> = HashSet::new();
        let u3: HashSet<u32> = HashSet::new();
        let mut u4: HashMap<Vec<String>, Traversal> = HashMap::new();
        u4.insert(s, trav);
        Self {
            start: start,
            end: end,
            children: u2,
            parents: u3,
            id: i,
            traversals: u4,
            core: core,

        }
    }


    /// Adding a children
    pub fn addChild(&mut self, new_child: u32){self.children.insert(new_child);
    }

    /// Adding a parent
    pub fn addPar(&mut self, new_parent: u32){
        self.parents.insert(new_parent);
    }

    /// Mean, max and min length of all traversals
    ///
    /// ??? Changing
    pub fn traversal_stats(&self) -> (u32, u32, f32){
        let mut all_length = Vec::new();
        for (k,v) in self.traversals.iter(){
            all_length.push(v.length);
        }

        let m1 = all_length.iter().max().unwrap();
        let m2 = all_length.iter().min().unwrap();
        let sum: u32 = all_length.iter().sum();

        let mean  = sum as f32 / all_length.len() as f32;
        (m1.clone(), m2.clone(),mean)
    }

    /// Number of traversal
    pub fn number_traversals(&self) -> usize{
        self.traversals.len()
    }

    /// Total number of intervals
    pub fn number_interval(&self) -> usize{
        let mut number = 0;
        for (k,v) in self.traversals.iter(){
            number += v.pos.len();
        }
        number
    }

    /// Number of different accessions
    pub fn number_acc(&self, hm: HashMap<u32, Posindex>) -> usize{
        let mut accession_numb= HashSet::new();
        for (k,v) in self.traversals.iter(){
            for x in v.pos.iter(){
                accession_numb.insert(hm.get(x).unwrap().acc.clone());
            }
        }
        accession_numb.len()
    }
}



pub fn bubble_naming_old(hm1: &HashMap<u32, Bubble>, s: &Vec<u32>, nodeid: &u32, maxcore: &u32, number: &u32, buff: & mut BufWriter<File>){
    let bubble = hm1.get(& nodeid).unwrap();
    let core = bubble.core;
    let mut h = s.clone();
    // Auffüllen
    for x in s.len() as u32..*maxcore-core{
        h.push(0);
    }
    h.push(number.clone());

    let mut h2 = h.clone();
    for x in 2..core{
        h2.push(0 as u32);
    }
    let j:Vec<String> = h2.iter().map(|i| i.to_string()).collect();
    write!(*buff, "{}\t{}\t{}\n", j.join("."), bubble.traversals.len(), bubble.number_interval()).expect("Can not write file");

    if bubble.children.len() > 0 {
        let mut it = 1;
        for x in bubble.children.iter(){

            bubble_naming_old(hm1, &h, x, maxcore, &it, buff);
            it += 1;
        }
    }
}

pub fn naming_wrapper(hm1: &HashMap<u32, Bubble>, maxcore: &u32, out: &str){
    let f = File::create([out, "stats"].join(".")).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    let mut it = 0;
    for (k,v) in hm1.iter(){

        if v.parents.len() == 0{
            let j: Vec<u32> = vec![];

            bubble_naming_old(hm1, &j, k, maxcore, &it, & mut f);

            it += 1;
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Traversal {
    pub length: u32, // Sequence length
    pub pos: Vec<u32>,
}

impl  Traversal{
    pub fn addPos(&mut self, pos: u32){
        self.pos.push(pos);
    }

    pub fn numbAcc(&self, tindex: &HashMap<u32, Posindex>){
        let mut h :HashSet<&String> = HashSet::new();
        for x in self.pos.iter(){
            h.insert(&tindex.get(x).unwrap().acc);
        }

    }

    pub fn new(posid: u32, lens: u32) -> Self{
        let mut k: Vec<u32> = Vec::new();
        k.push(posid);
        Self{
            length: lens,
            pos: k,
        }
    }

    pub fn get_pos(& self) -> usize{
        self.pos.len()
    }
}



