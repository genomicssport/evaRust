use crate::structfile::HPOOMIM;
use std::error::Error;
use std::io::{BufRead, BufReader};
/*
Author Gaurav Sablok
Instytut Chemii Bioorganicznej
Polskiej Akademii Nauk
ul. Noskowskiego 12/14 | 61-704, Poznań
Date: 2025-7-23
*/

#[tokio::main]
pub async fn hpoomimmap(hpo: &str) -> Result<Vec<HPOOMIM>, Box<dyn Error>> {
    let fileopen = std::fs::File::open(hpo).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let returnvector: Vec<Vec<_>> = fileread
        .lines()
        .filter_map(|line: Result<String, _>| line.ok())
        .par_bridge()
        .map(|x| mapiter(x).unwrap())
        .collect::<Vec<_>>();
    let mut finaljson: Vec<HPOOMIM> = Vec::new();
    for i in returnvector.iter() {
        for val in i.iter() {
            finaljson.push(HPOOMIM {
                omimcui: val.omimcui.clone(),
                mimnumber: val.mimnumber.clone(),
                omimname: val.omimname.clone(),
                relationship: val.relationship.clone(),
                hpocui: val.hpocui.clone(),
                hpoid: val.hpoid.clone(),
                hponame: val.hponame.clone(),
                medgenname: val.medgenname.clone(),
                medgensource: val.medgensource.clone(),
                sty: val.sty.clone(),
            });
        }
    }
    Ok(finaljson)
}

pub fn mapiter(lineread: String) -> std::io::Result<Vec<HPOOMIM>> {
    let mut medgenpubmed: Vec<HPOOMIM> = Vec::new();
    let line = lineread.clone();
    if !line.starts_with("#") {
        let linesplit: Vec<_> = line.split("|").map(String::from).collect::<Vec<_>>();
        medgenpubmed.push(HPOOMIM {
            omimcui: linesplit[0].to_string(),
            mimnumber: linesplit[1].to_string(),
            omimname: linesplit[2].to_string(),
            relationship: linesplit[3].to_string().split(":").collect::<Vec<_>>()[1]
                .to_string()
                .clone(),
            hpocui: linesplit[4].to_string(),
            hpoid: linesplit[5].to_string(),
            hponame: linesplit[6].to_string(),
            medgenname: linesplit[7].to_string(),
            medgensource: linesplit[8].to_string(),
            sty: linesplit[9].to_string(),
        });
    }
    Ok(medgenpubmed)
}
