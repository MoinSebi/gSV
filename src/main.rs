

mod core;
#[allow(non_snake_case)]
mod panSV;
mod bifurcation;

use crate::core::counting::{CountNode};
use crate::panSV::algo::{algo_panSV, create_bubbles, indel_detection, check_bubble_size};
use crate::core::graph_helper::graph2pos;
use clap::{Arg, App};
use std::path::Path;
use std::process;
use crate::panSV::panSV_core::OldNaming;
use gfaR_wrapper::{NGfa, GraphWrapper};
use crate::core::writer::{writing_traversals, writing_bed, bubble_naming_new, bubble_naming_old, bubble_parent_structure, writing_uniques_bed, writing_bed_traversals};


fn main() {
    let matches = App::new("panSV")
        .version("0.1.0")
        .author("Sebastian V")
        .about("Bubble detection")
        .arg(Arg::new("gfa")
            .short('g')
            .long("gfa")
            .about("Input GFA file")
            .takes_value(true)
            .required(true))

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
        .arg(Arg::new("old naming")
            .short('n')
            .long("naming")
            .about("Change the naming"))
        .arg(Arg::new("delimiter")
            .short('d')
            .long("delimiter")
            .about("Delimiter for between genome and chromosome")
            .takes_value(true))
        .arg(Arg::new("unique")
            .short('u')
            .long("unique")
            .about("return a bed file with only unique sequences for each bubble")
            .takes_value(true))



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
    println!("Counting nodes");

    let mut counts: CountNode = CountNode::new();
    if matches.is_present("delimiter"){
        let mut h: GraphWrapper = GraphWrapper::new();
        h.fromNGfa(&graph, matches.value_of("delimiter").unwrap());
        eprintln!("{} Genomes and {} Paths", h.genomes.len(), graph.paths.len());
        eprintln!("Counting nodes");
        counts.counting_wrapper(&graph, &h);
    } else {
        eprintln!("{} Genomes and {} Paths", graph.paths.len(), graph.paths.len());
        eprintln!("Counting nodes");
        counts.counting_graph(&graph);
    }

    // test

    let (o,_m) = algo_panSV(&graph.paths, &counts);
    let h = graph2pos(&graph);
    let mut gg = create_bubbles(&o, &graph.paths, &h);


    eprintln!("\nIndel detection");
    let interval_numb = gg.id2interval.len() as u32;
    indel_detection(& mut gg, &graph.paths, interval_numb);

    eprintln!("\nCategorize bubbles");
    check_bubble_size(&mut gg);


    let mut jj = OldNaming::new();


    eprintln!("Writing stats");
    if matches.is_present("old naming"){
        bubble_naming_old(&gg.id2bubble, & mut jj.hm, outpre, &(graph.paths.len() as u32));
    } else {
        bubble_naming_new(&gg.id2bubble, outpre);
        bubble_parent_structure(&gg.id2bubble, outpre);
    }



    eprintln!("Writing bed");
    writing_bed(& gg, &h, outpre);
    writing_bed_traversals(&gg, &h, outpre);


    if matches.is_present("traversal"){
        eprintln!("Writing traversal");
        writing_traversals(&gg, outpre);
    }

    if matches.is_present("unique"){
        eprintln!("Writing traversal");
        let size: usize = matches.value_of("unique").unwrap().parse().unwrap();
        writing_uniques_bed(&gg, &h, outpre , size);
    }



}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::writer::writing_bed_traversals;
    use crate::panSV::algo::check_bubble_size;

    #[test]
    fn counting() {
        let mut graph: NGfa = NGfa::new();
        graph.from_graph("example_data/testGraph.gfa");
        let mut counts: CountNode = CountNode::new();
        counts.counting_graph(&graph);
        assert_eq!(counts.ncount.len(), 9);
        assert_eq!(counts.ncount.contains_key(&1), true);

    }

    #[test]
    fn detection() {
        let mut graph: NGfa = NGfa::new();
        graph.from_graph("example_data/testGraph.gfa");
        let mut counts: CountNode = CountNode::new();
        counts.counting_graph(&graph);
        let (o,_m) = algo_panSV(&graph.paths, &counts);
        let h = graph2pos(&graph);
        let mut gg = create_bubbles(&o, &graph.paths, &h);
        let interval_numb = gg.id2interval.len() as u32;
        indel_detection(& mut gg, &graph.paths, interval_numb);
        println!("djaskdjakjdkasjsdsa");
        check_bubble_size(&mut gg);
        bubble_naming_new(&gg.id2bubble, "example_data/panSV_test2");

    }

    #[test]
    fn writing() {
        let mut graph: NGfa = NGfa::new();
        graph.from_graph("example_data/testGraph.gfa");
        let mut counts: CountNode = CountNode::new();
        counts.counting_graph(&graph);
        let (o,_m) = algo_panSV(&graph.paths, &counts);
        let h = graph2pos(&graph);
        let mut gg = create_bubbles(&o, &graph.paths, &h);
        let interval_numb = gg.id2interval.len() as u32;
        indel_detection(& mut gg, &graph.paths, interval_numb);
        bubble_naming_new(&gg.id2bubble, "example_data/panSV_test");
        bubble_parent_structure(&gg.id2bubble, "example_data/panSV_test");
        writing_traversals(&gg, "example_data/panSV_test");
        writing_uniques_bed(&gg, &h, "example_data/panSV_test" , 50);
        writing_bed(& gg, &h, "example_data/panSV_test");
        writing_bed_traversals(&gg, &h, "example_data/panSV_test");
    }


}
