

mod core;
#[allow(non_snake_case)]
mod panSV;
mod bifurcation;

use std::collections::HashMap;
use crate::core::counting::{CountNode};
use crate::panSV::algo::{algo_panSV, create_bubbles, indel_detection, check_bubble_size, nest, sort_trav};
use crate::core::graph_helper::graph2pos;
use clap::{Arg, App, AppSettings};
use std::path::Path;
use std::process;
use env_logger::{Builder, Target};
use crate::panSV::panSV_core::{BubbleWrapper, OldNaming, PanSVpos};
use gfaR_wrapper::{NGfa, GraphWrapper};
use log::{info, LevelFilter, warn};
use crate::bifurcation::algo::{create_bubbles2, bifurcation_wrapper};
use crate::core::writer::{writing_traversals, writing_bed, bubble_naming_new, bubble_naming_old, bubble_parent_structure, writing_uniques_bed, writing_bed_traversals, writing_uniques_bed_stats};
use std::io::Write;
use chrono::Local;

fn main() {
    let matches = App::new("panSV")
        .setting(AppSettings::ArgRequiredElseHelp)
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
        .arg(Arg::new("bifurcation")
            .short('b')
            .long("bifurcation")
            .about("Bifurcation mode "))
        .arg(Arg::new("traversal")
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
            .about("Return additional files with unique traversals above THIS value")
            .default_value("50")
            .takes_value(true))

        .arg(Arg::new("verbose")
            .short('v')
            .long("verbose")
            .about("Verbose "))
        .arg(Arg::new("threads")
            .short('t')
            .long("threads")
            .about("Number of threads")
            .default_value("1"))



        .get_matches();



    // Checking verbose
    if matches.is_present("verbose"){
        Builder::new()
            .format(|buf, record| {
                writeln!(buf,
                         "{} [{}] - {}",
                         Local::now().format("%Y-%m-%dT%H:%M:%S"),
                         record.level(),
                         record.args()
                )
            })
            .filter(None, LevelFilter::Trace)
            .target(Target::Stderr)
            .init();
    } else {
        Builder::new()
            .format(|buf, record| {
                writeln!(buf,
                         "{} [{}] - {}",
                         Local::now().format("%Y-%m-%dT%H:%M:%S"),
                         record.level(),
                         record.args()
                )
            })
            .filter(None, LevelFilter::Info)
            .target(Target::Stderr)
            .init();
    }


    info!("Running gSV");
    let threads= matches.value_of("threads").unwrap().parse().unwrap();


    let mut g1 = "not_relevant";
    if matches.is_present("gfa") {
        if Path::new(matches.value_of("gfa").unwrap()).exists() {
            g1 = matches.value_of("gfa").unwrap();
        } else {
            warn!("No file with such name");
            process::exit(0x0100);
        }

    }



    let outprefix= matches.value_of("output").unwrap();



    let mut graph: NGfa = NGfa::new();
    graph.from_graph(g1);

    // Counting nodes


    // test

    let mut gg: BubbleWrapper;
    let mut o: HashMap<String, Vec<PanSVpos>>;
    let h = graph2pos(&graph);
    let mut counts: CountNode = CountNode::new();


    if matches.is_present("bifurcation"){
        o = bifurcation_wrapper(&graph, &threads);

        gg = create_bubbles(& o, & graph.paths, &h);
    } else {
        let mut counts: CountNode = CountNode::new();
        info!("Counting nodes");
        if matches.is_present("delimiter"){
            let mut h: GraphWrapper = GraphWrapper::new();
            h.fromNGfa(&graph, matches.value_of("delimiter").unwrap());
            info!("{} Genomes and {} Paths", h.genomes.len(), graph.paths.len());
            info!("Counting nodes");
            counts.counting_wrapper(&graph, &h);
        } else {
            info!("{} Genomes and {} Paths", graph.paths.len(), graph.paths.len());
            info!("Counting nodes");
            counts.counting_graph(&graph);
        }
        o = algo_panSV(&graph.paths, &counts).0;
        gg = create_bubbles(&o, &graph.paths, &h);
        info!("Indel detection");
        let interval_numb = gg.id2interval.len() as u32;
        indel_detection(& mut gg, &graph.paths, interval_numb);
    }




    info!("Categorize bubbles");
    check_bubble_size(&mut gg);
    nest(& mut gg);

    let mut jj = OldNaming::new();


    info!("Writing stats");
    if matches.is_present("old naming"){
        bubble_naming_old(&gg.id2bubble, & mut jj.hm, outprefix, &(graph.paths.len() as u32));
    } else {
        bubble_naming_new(&gg.id2bubble, outprefix);
        bubble_parent_structure(&gg.id2bubble, outprefix);
    }



    info!("Writing bed");
    writing_bed(& gg, &h, outprefix);
    writing_bed_traversals(&gg, &h, outprefix);


    if matches.is_present("traversal"){
        info!("Writing traversal");
        writing_traversals(&gg, outprefix);
    }

    if matches.is_present("unique"){
        info!("Writing traversal");
        let size: usize = matches.value_of("unique").unwrap().parse().unwrap();
        writing_uniques_bed(&gg, &h, outprefix, size);
        writing_uniques_bed_stats(&gg, &h, outprefix, size);
    }



}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::writer::writing_bed_traversals;
    use crate::panSV::algo::{check_bubble_size, nest, check_nest};

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
        nest(& mut gg);
        check_nest(& mut gg);
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
