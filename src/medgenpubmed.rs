use crate::structfile::MedgenPubMed;
use rayon::prelude::*;
use std::error::Error;
use std::io::{BufRead, BufReader};

/*
Author Gaurav Sablok
Instytut Chemii Bioorganicznej
Polskiej Akademii Nauk
ul. Noskowskiego 12/14 | 61-704, Poznań
Date: 2025-7-23
*/

pub fn medgenpubmedmap(pubmedstring: &str) -> Result<Vec<MedgenPubMed>, Box<dyn Error>> {
    let fileopen = std::fs::File::open(pubmedstring).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let returnvector: Vec<Vec<_>> = fileread
        .lines()
        .filter_map(|line: Result<String, _>| line.ok())
        .par_bridge()
        .map(|x| mapiter(x).unwrap())
        .collect::<Vec<_>>();

    let mut finaljson: Vec<MedgenPubMed> = Vec::new();
    for i in returnvector.iter() {
        for j in i.iter() {
            finaljson.push(MedgenPubMed {
                uid: j.uid.clone(),
                cui: j.cui.clone(),
                name: j.name.clone(),
                pmid: j.pmid.clone(),
            });
        }
    }
    Ok(finaljson)
}

pub fn mapiter(lineread: String) -> std::io::Result<Vec<MedgenPubMed>> {
    let mut medgenpubmed: Vec<MedgenPubMed> = Vec::new();
    let line = lineread.clone();
    if !line.starts_with("#") {
        let linesplit: Vec<_> = line.split("|").map(String::from).collect::<Vec<_>>();
        medgenpubmed.push(MedgenPubMed {
            uid: linesplit[0].to_string().clone(),
            cui: linesplit[1].to_string().clone(),
            name: linesplit[2].to_string().clone(),
            pmid: linesplit[3].to_string().clone(),
        });
    }
    Ok(medgenpubmed)
}
