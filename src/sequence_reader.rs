
use std::{fs::read_to_string, path::PathBuf};
use std::sync::Arc;
use regex::Regex;

#[derive(Debug)]
pub struct Fasta {
    pub id: Arc<str>,
    pub sequence: Arc<str>,
    pub species: Arc<str>,
    pub species_id: Arc<str>,
    pub description: Arc<str>,
}

impl Fasta {

    fn from_vec(vec:Vec<&str>) -> Result<Self, &str> {
        if vec.len() == 5 {
            let [id, sequence, species, species_id, decription] = vec[..] else {panic!("Rubish fasta")};
            Ok(Fasta {
                id: id.into(),
                sequence: sequence.into(),
                species: species.into(),
                species_id: species_id.into(),
                description: decription.into(),
            })
        } else {
            Err("Reformat fasta to: id, ")
        }
    }
}



pub fn read_peptides(path: PathBuf) -> Arc<[Arc<str>]>{
    read_to_string(path)
        .expect("Error reading peptide list")
        .lines()
        .map(|line| -> Arc<str> {
            line.trim().into()
        }).collect::<Vec<Arc<str>>>().into()
}


pub fn read_fasta_csv(path:PathBuf) -> Arc<[Fasta]>{
        let patt = Regex::new(r"|[A-Z0-9]{6}|").unwrap();
        let data = read_to_string(path)
        .expect("Error reading fasta file")
        .lines()
        .filter(|line|{
            patt.is_match(line)
        })
        .map(|line| {
            line.split("\t")
                .collect::<Vec<&str>>()})
        .map(
            |item |{
                Fasta::from_vec(item).unwrap()
            }
        )
        .collect::<Vec<Fasta>>()
        .into();
        return data;
    }

