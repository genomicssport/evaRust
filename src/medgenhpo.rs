use crate::structfile::MedgenHPO;
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

pub fn medgenhpomap(pubmedstring: &str) -> Result<Vec<MedgenHPO>, Box<dyn Error>> {
    let fileopen = std::fs::File::open(pubmedstring).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let returnvector: Vec<Vec<_>> = fileread
        .lines()
        .filter_map(|line: Result<String, _>| line.ok())
        .par_bridge()
        .map(|x| mapiter(x).unwrap())
        .collect::<Vec<_>>();

    let mut finaljson: Vec<MedgenHPO> = Vec::new();
    for i in returnvector.iter() {
        for val in i.iter() {
            finaljson.push(MedgenHPO {
                cui: val.cui.clone(),
                sdui: val.sdui.clone(),
                hpostr: val.hpostr.clone(),
                medgenstr: val.medgenstr.clone(),
                medgenstrsab: val.medgenstrsab.clone(),
                sty: val.sty.clone(),
            });
        }
    }
    Ok(finaljson)
}

pub fn mapiter(lineread: String) -> std::io::Result<Vec<MedgenHPO>> {
    let mut medgenhpo: Vec<MedgenHPO> = Vec::new();
    let line = lineread.clone();
    if !line.starts_with("#") {
        let linesplit: Vec<_> = line.split("|").map(String::from).collect::<Vec<_>>();
        medgenhpo.push(MedgenHPO {
            cui: linesplit[0].to_string().clone(),
            sdui: linesplit[1].to_string().clone(),
            hpostr: linesplit[2].to_string().split(":").collect::<Vec<_>>()[1]
                .to_string()
                .clone(),
            medgenstr: linesplit[3].to_string().clone(),
            medgenstrsab: linesplit[4].to_string().clone(),
            sty: linesplit[5].to_string().clone(),
        });
    }
    Ok(medgenhpo)
}
