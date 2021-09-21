

mod core;
#[allow(non_snake_case)]
mod panSV;
use crate::core::counting::{counting_nodes, CountNode};
use crate::panSV::algo::{algo_panSV, create_bubbles, indel_detection, writing_bed, writing_traversals};
use crate::core::graph_helper::graph2pos;
use crate::core::core::{naming_wrapper};
use clap::{Arg, App};
use std::path::Path;
use std::process;
use crate::panSV::panSV_core::OldNaming;
use gfaR_wrapper::NGfa;


fn main() {
    let matches = App::new("panSV")
        .version("0.1.0")
        .author("Sebastian V")
        .about("Bubble detection")
        .arg(Arg::new("gfa")
            .short('g')
            .long("gfa")
            .about("Input GFA file")
            .takes_value(true))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .about("Output prefix")
            .takes_value(true)
            .default_value("panSV.output"))
        .arg(Arg::new("traversal")
            .short('t')
            .long("traversal")
            .about("Additional traversal file as output"))

        .get_matches();




    let g1;
    if matches.is_present("gfa") {
        if Path::new(matches.value_of("gfa").unwrap()).exists() {
            g1 = matches.value_of("gfa").unwrap();
        } else {
            eprintln!("No input gfa file");
            process::exit(0x0100);
        }

    } else {
        eprintln!("No input gfa file");
        process::exit(0x0100);
    }



    let outpre;
    if matches.is_present("output"){
        outpre = matches.value_of("output").unwrap();
    } else {
        outpre = "panSV.out"
    }



    let mut graph: NGfa = NGfa::new();
    graph.from_graph(g1);

    // Counting nodes
    println!("Counting");

    let gg: CountNode = counting_nodes(&graph);



    println!("PanSV running");
    let (o,m) = algo_panSV(&graph.paths, &gg);
    let h = graph2pos(&graph);
    println!("Creating bubbles");
    let mut gg = create_bubbles(&o, &graph.paths, &h);
    //let jo = gg.id2interval.keys().into_iter().max().unwrap().clone();

    println!("Indel detection");
    let interval_numb = gg.id2interval.len() as u32;
    indel_detection(& mut gg, &graph.paths, interval_numb);

    let mut jj = OldNaming::new();


    println!("Writing bed");
    naming_wrapper(& gg.id2bubble, &(graph.paths.len() as u32), outpre,  &mut jj.hm);
    println!("Writing stats");
    writing_bed(& gg, &h, &m, outpre);


    if matches.is_present("traversal"){
        println!("Writing traversal");
        writing_traversals(&gg, &jj, outpre);
    }


}

fn running(){
    println!("Jdkjasdhsakjd");
}