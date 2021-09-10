

mod core;
mod panSV;
use gfaR::{Gfa};
use crate::core::counting::{counting_nodes, count_node};
use crate::panSV::algo::{algo_panSV, createBubbles, mapper1, indel_detection, bed};
use crate::core::graph_helper::graph2pos;
use crate::core::core::{naming_wrapper};
use clap::{Arg, App};

fn main() {
    let matches = App::new("panSV")
        .version("0.1.0")
        .author("Sebastian V")
        .about("Bubble detection")
        .arg(Arg::new("gfa2")
            .short('g')
            .long("gfa")
            .about("Sets the input file to use")
            .takes_value(true)
            .default_value("/home/svorbrugg_local/Rust/data/AAA_AAB.cat.gfa"))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .about("Output prefix")
            .takes_value(true)
            .default_value("panSV.output"))

        .get_matches();

    // removed .default stuff
    let _input = matches.value_of("gfa2").unwrap();
    let _output: &str = matches.value_of("output").unwrap();



    let mut g1;
    if matches.is_present("gfa2"){
        g1 = matches.value_of("gfa2").unwrap();
    } else {
        g1 = "/home/svorbrugg_local/local_compile/panSV/graphs/testGraph.gfa";
        let g2 = "/home/svorbrugg_local/local_compile/panSV/graphs/openGraph.gfa";
        let g3 = "/home/svorbrugg_local/Rust/data/AAA_AAB.cat.gfa";
        let g4 = "/home/svorbrugg_local/chr1.wfmash.n20.a90.s10000.p1,19,39,3,81,1.seqwish.sort.smooth.sort.gfa";
    }


    let mut outpre;
    if matches.is_present("output"){
        outpre = matches.value_of("output").unwrap();
    } else {
        outpre = "panSV.out"
    }

    let mut graph: Gfa = Gfa::new();
    graph.read_file(g1);

    // Counting nodes
    println!("Counting");
    let gg: count_node = counting_nodes(&graph);
    let h = graph2pos(&graph);

    println!("Number of nodes: {}", gg.ncount.len());
    let (o,m) = algo_panSV(&graph.paths, &gg);
    let k = mapper1(&o);


    let mut gg = createBubbles(&o, &graph.paths, &h);
    //let jo = gg.id2interval.keys().into_iter().max().unwrap().clone();
    indel_detection(& mut gg, &graph.paths, 100000);

    naming_wrapper(& gg.id2bubble, &(graph.paths.len() as u32), outpre);
    bed(& gg, &h,&m, outpre);







}
