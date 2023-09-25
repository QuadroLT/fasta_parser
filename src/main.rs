
use std::io::Write;
use std::path::PathBuf;
use std::fs::File;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use clap::{Arg, App};
use rayon::prelude::*;
use indicatif::{ParallelProgressIterator, ProgressStyle, ProgressIterator};
use std::time::Instant;
mod sequence_reader;


fn main() {
    // arg parse interface
    let args = App::new("Peptide analyzer")
        .version("0.1a")
        .author("QuadroLT")
        .about("Takes list of peptides and checks for belonging to proteins in SwissProt protein collections. Spits out CSV of peptide protein matches")
        .arg(
            Arg::new("peptides as list")
                .id("peptides")
                .short('p')
                .long("peptides")
                .value_name("peptides")
                .help("path to peptides list")
                .required(true)
                .takes_value(true))
        .arg(
            Arg::new("fasta")
                .id("fasta")
                .short('f')
                .long("fasta")
                .value_name("fasta")
                .help("path to fasta file")
                .required(true))
        .arg(
            Arg::new("output")
                .id("output")
                .short('o')
                .long("output")
                .value_name("output")
                .help("path to output file")
                .required(true))
        .get_matches();

    // message passing
    let (tx, rx) = mpsc::channel::<String>();
    let tx = Arc::new(Mutex::new(tx)); // No other way to pass item between threaads
    // progress bar
    let style = ProgressStyle::default_bar();
    println!("Reading input");
    let peptides = args.get_one::<String>("peptides").unwrap();
    let peptides = PathBuf::from(peptides);
    let peptides = sequence_reader::read_peptides(peptides);
    let fasta  = args.get_one::<String>("fasta").unwrap();
    let fasta = PathBuf::from(fasta);
    let fasta = sequence_reader::read_fasta_csv(fasta);
    println!("Parsing fasta content");
    let start_time = Instant::now();
    peptides.par_iter()
        .progress_with_style(style.clone())
        .for_each(|peptide| {
            let txx = tx.clone();
            fasta.iter()
                .for_each(|prot| {
                    if prot.sequence.contains(&peptide.to_string()){
                        let sender  = txx.lock().unwrap();
                        let rv = format!("{}\t{}\t{}\t{}\n", peptide, prot.id, prot.species, prot.species_id);
                        sender.send(rv).unwrap();
                    }
                });
        });

    drop(tx); // finish messaging

    let end_time = Instant::now();
    let duration = end_time - start_time;
    println!("Parsing operation took {:?}", duration);
    // let result = rx.recv().iter().map(|x| x.to_owned()).collect::<Vec<String>>();
    // for item in result{
        // println!("{}", item);
    // }

    let output = args.get_one::<String>("output").unwrap();
    let output = PathBuf::from(output);
    let results = rx.iter().collect::<Vec<String>>();
    let mut file = File::create(output).expect("Output file exists!!!");
    let header = format!("{}\t{}\t{}\t{}\n", "peptide", "prot_id", "prot_species", "prot_species_id");
    write!(file, "{}", header).unwrap();
    for x in results.iter().progress_with_style(style.clone()){
        write!(file, "{}", x).unwrap();
    }
    file.flush().expect("Error writing file");
}

