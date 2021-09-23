use std::collections::{HashMap};
use crate::core::core::{Bubble, Posindex};


#[derive(Debug, Clone)]
pub struct PanSVpos {
    pub start:  u32,
    pub end:  u32,
    pub core: u32,
}

/// For interval_open ->
pub struct TmpPos {
    pub acc:  String,
    pub start:  u32,
    pub core:  u32,
}



pub struct OldNaming {
    pub hm: HashMap<u32, Vec<u32>>,
}

impl OldNaming {
    pub fn new()->Self{
        let g:  HashMap<u32, Vec<u32>> = HashMap::new();
        Self{
            hm: g,
        }
    }
}

pub struct BubbleWrapper<'a>{
    pub id2bubble: HashMap<u32, Bubble>,
    pub id2interval: HashMap<u32, Posindex>,

    pub anchor2bubble: HashMap<(&'a  u32, &'a u32), u32>,
    pub anchor2interval: HashMap<(&'a  u32, &'a  u32,&'a String), u32>,
    pub id2id: HashMap<(u32, u32, &'a  String), u32>,

}

impl BubbleWrapper<'_>{
    pub fn new() -> Self {
        let id2bubble: HashMap<u32, Bubble> = HashMap::new();
        let id2interval: HashMap<u32, Posindex> = HashMap::new();
        let anchor2bubble: HashMap<(& u32, & u32), u32> = HashMap::new();
        let anchor2interval: HashMap<(& u32, & u32, & String), u32> = HashMap::new();
        let id2id: HashMap<(u32, u32, & String), u32> = HashMap::new();

        Self{
            id2id: id2id,
            id2bubble: id2bubble,
            id2interval: id2interval,
            anchor2bubble: anchor2bubble,
            anchor2interval: anchor2interval,
        }
    }
}