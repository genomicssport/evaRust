use crate::structfile::MedgenMap;
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

pub fn medgenmapmap(pubmedstring: &str) -> Result<Vec<MedgenMap>, Box<dyn Error>> {
    let fileopen = std::fs::File::open(pubmedstring).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let returnvector: Vec<Vec<_>> = fileread
        .lines()
        .filter_map(|line: Result<String, _>| line.ok())
        .par_bridge()
        .map(|x| mapiter(x).unwrap())
        .collect::<Vec<_>>();

    let mut finaljson: Vec<MedgenMap> = Vec::new();
    for i in returnvector.iter() {
        for j in i.iter() {
            finaljson.push(MedgenMap {
                cuiid: j.cuiid.clone(),
                prefname: j.prefname.clone(),
                sourceid: j.sourceid.clone(),
                source: j.source.clone(),
            });
        }
    }
    Ok(finaljson)
}
pub fn mapiter(lineread: String) -> std::io::Result<Vec<MedgenMap>> {
    let mut medgenpubmed: Vec<MedgenMap> = Vec::new();
    let line = lineread.clone();
    if !line.starts_with("#") {
        let linesplit: Vec<_> = line.split("|").map(String::from).collect::<Vec<_>>();
        medgenpubmed.push(MedgenMap {
            cuiid: linesplit[0].to_string().clone(),
            prefname: linesplit[1].to_string().clone(),
            sourceid: linesplit[2].to_string().clone(),
            source: linesplit[3].to_string().clone(),
        });
    }
    Ok(medgenpubmed)
}
