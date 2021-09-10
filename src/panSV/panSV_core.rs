use std::collections::{HashMap, HashSet};
use crate::core::core::{bubble, posindex, Traversal};


#[derive(Debug, Clone)]
pub struct panSVpos{
    pub start: u32,
    pub end: u32,
    pub core: u32
}

/// For interval_open ->
pub struct tmp_pos{
    pub acc: String,
    pub start: u32,
    pub core: u32,
}


pub struct index<'a> {
    //
    pub anchor2bubble: HashMap<(&'a String, &'a String), u32>,
    pub bubbleAnchor_set:  HashSet<(&'a String, &'a String)>,
    pub t2bubble: HashMap<posindex, u32>,
    pub t2t: HashMap<posindex, u32>,
    pub trans2bubble: HashMap<u32, u32>,
    pub traversalhm: HashMap<u32, Vec<String>>,
}

impl <'a> index<'a>{
    pub fn new() -> Self{
        let t2b: HashMap<u32, u32> = HashMap::new();
        let bubbleAnchor_set:  HashSet<(&'a String, &'a String)> = HashSet::new();
        let k: HashMap<u32, Vec<String>>   = HashMap::new();
        let tt: HashMap<(&'a String, &'a String), u32> = HashMap::new();
        let t2: HashMap<posindex, u32> = HashMap::new();
        let t3: HashMap<posindex, u32> = HashMap::new();
        Self {
            trans2bubble: t2b,
            bubbleAnchor_set: bubbleAnchor_set,
            anchor2bubble: tt,
            t2bubble: t2,
            t2t: t3,
            traversalhm: k,
        }
    }
}


pub struct panSVout {
    pub id2bubble: HashMap<u32,  bubble>,
    pub id2tran: HashMap<u32, posindex>,
}


impl panSVout{
    pub fn new() -> Self{
        let t2b: HashMap<u32, bubble> = HashMap::new();
        let i2t:  HashMap<u32, posindex> = HashMap::new();
        Self {
            id2bubble: t2b,
            id2tran: i2t,
        }
    }
}