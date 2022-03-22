

mod core;
#[allow(non_snake_case)]
mod panSV;
mod bifurcation;

use std::collections::HashMap;
use crate::core::counting::{CountNode};
use crate::panSV::algo::{algo_panSV, create_bubbles, indel_detection, check_bubble_size, nest_wrapper, sort_trav, nest_version2};
use crate::core::graph_helper::graph2pos;
use clap::{Arg, App, AppSettings};
use std::path::Path;
use std::process;
use env_logger::{Builder,Target};
use crate::panSV::panSV_core::{BubbleWrapper, OldNaming, PanSVpos};
use gfaR_wrapper::{NGfa, GraphWrapper};
use log::{info, LevelFilter, warn};
use crate::bifurcation::algo::{bifurcation_wrapper};
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
        .arg(Arg::new("Nestedness")
            .long("nestedness")
            .about("Add nestedness to the stats output"))
        .arg(Arg::new("verbose")
            .short('v')
            .about("-v = DEBUG | -vv = TRACE")
            .takes_value(true)
            .default_missing_value("v1"))
        .arg(Arg::new("quiet")
            .short('q')
            .about("No updating INFO messages"))
        .arg(Arg::new("threads")
            .short('t')
            .long("threads")
            .about("Number of threads")
            .default_value("1"))



        .get_matches();

    // Checking verbose
    // Ugly, but needed - May end up in a small library later
    if matches.is_present("quiet"){
        Builder::new()
            .format(|buf, record| {
                writeln!(buf,
                         "{} [{}] - {}",
                         Local::now().format("%Y-%m-%dT%H:%M:%S"),
                         record.level(),
                         record.args()
                )
            })
            .filter(None, LevelFilter::Warn)
            .target(Target::Stderr)
            .init();

    }

    else if matches.is_present("verbose"){
        if matches.value_of("verbose").unwrap() == "v1"{
            Builder::new()
                .format(|buf, record| {
                    writeln!(buf,
                             "{} [{}] - {}",
                             Local::now().format("%Y-%m-%dT%H:%M:%S"),
                             record.level(),
                             record.args()
                    )
                })
                .filter(None, LevelFilter::Debug)
                .target(Target::Stderr)
                .init();
        }
        else if matches.value_of("verbose").unwrap() == "v"{
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
        }
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
    let o: HashMap<String, Vec<PanSVpos>>;
    let h = graph2pos(&graph);


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

    if matches.is_present("Nestedness"){
        info!("Nestedness");
        nest_version2(& mut gg);
    }

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

