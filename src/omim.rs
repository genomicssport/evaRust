use crate::hpoomim::hpoomimmap;
use crate::medgenhpo::medgenhpomap;
use crate::structfile::HPOOMIM;
use crate::structfile::MedgenHPO;
use crate::structfile::OMIMEvidence;
use crate::structfile::OMIMEvidenceMerged;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Author Gaurav Sablok
Instytut Chemii Bioorganicznej
Polskiej Akademii Nauk
ul. Noskowskiego 12/14 | 61-704, Poznań
Date: 2025-7-23
*/

#[tokio::main]
pub async fn omimevidence(
    pathomim: &str,
    evidence: &str,
    medgenomim: &str,
    medgenhpo: &str,
) -> Result<Vec<OMIMEvidenceMerged>, Box<dyn Error>> {
    let omimevidence = File::open(pathomim).expect("file not found");
    let omimevidenceread = BufReader::new(omimevidence);
    let omimevidence: Vec<Vec<_>> = omimevidenceread
        .lines()
        .filter_map(|line: Result<String, _>| line.ok())
        .par_bridge()
        .map(|x| mapiter(x).unwrap())
        .collect::<Vec<_>>();
    let mut finaljson: Vec<OMIMEvidence> = Vec::new();
    for i in omimevidence.iter() {
        for val in i.iter() {
            finaljson.push(OMIMEvidence {
                databaseid: val.databaseid.clone(),
                diseasename: val.diseasename.clone(),
                qualifier: val.qualifier.clone(),
                hpoid: val.hpoid.clone(),
                reference: val.reference.clone(),
                evidence: val.evidence.clone(),
                onset: val.onset.clone(),
                frequency: val.frequency.clone(),
                sex: val.sex.clone(),
                modifier: val.modifier.clone(),
                aspect: val.aspect.clone(),
                biocuration: val.biocuration.clone(),
            });
        }
    }
    let hpomap: Vec<HPOOMIM> = hpoomimmap(medgenomim).unwrap();
    let hpomedgen: Vec<MedgenHPO> = medgenhpomap(medgenhpo).unwrap();
    let mut annotatevariant: Vec<OMIMEvidenceMerged> = Vec::new();
    for i in finaljson.iter() {
        for j in hpomap.iter() {
            for val in hpomedgen.iter() {
                if i.hpoid.parse::<usize>().unwrap() == evidence.parse::<usize>().unwrap() {
                    annotatevariant.push(OMIMEvidenceMerged {
                        databaseid: i.databaseid.clone(),
                        diseasename: i.diseasename.clone(),
                        qualifier: i.qualifier.clone(),
                        hpoid: i.hpoid.clone(),
                        reference: i.reference.clone(),
                        evidence: i.evidence.clone(),
                        onset: i.onset.clone(),
                        frequency: i.frequency.clone(),
                        sex: i.sex.clone(),
                        modifier: i.modifier.clone(),
                        aspect: i.aspect.clone(),
                        biocuration: i.biocuration.clone(),
                        cui: val.cui.clone(),
                        sdui: val.sdui.clone(),
                        hpostr: val.hpostr.clone(),
                        medgenstr: val.medgenstr.clone(),
                        medgenstrsab: val.medgenstrsab.clone(),
                        sty: val.sty.clone(),
                        omimcui: j.omimcui.clone(),
                        mimnumber: j.mimnumber.clone(),
                        omimname: j.omimname.clone(),
                        relationship: j.relationship.clone(),
                        hpocui: j.hpocui.clone(),
                        hponame: j.hponame.clone(),
                        medgenname: j.medgenname.clone(),
                        medgensource: j.medgensource.clone(),
                    });
                }
            }
        }
    }
    Ok(annotatevariant)
}

pub fn mapiter(lineread: String) -> std::io::Result<Vec<OMIMEvidence>> {
    let mut medgenpubmed: Vec<OMIMEvidence> = Vec::new();
    let line = lineread.clone();
    if !line.starts_with("#") {
        let linesplit: Vec<_> = line.split("|").map(String::from).collect::<Vec<_>>();
        medgenpubmed.push(OMIMEvidence {
            databaseid: linesplit[0].to_string().clone(),
            diseasename: linesplit[1].to_string().clone(),
            qualifier: linesplit[2].to_string().clone(),
            hpoid: linesplit[3].split(":").collect::<Vec<_>>()[1]
                .to_string()
                .clone(),
            reference: linesplit[4].to_string().clone(),
            evidence: linesplit[5].to_string().clone(),
            onset: linesplit[6].to_string().clone(),
            frequency: linesplit[7].to_string().clone(),
            sex: linesplit[8].to_string().clone(),
            modifier: linesplit[9].to_string().clone(),
            aspect: linesplit[10].to_string().clone(),
            biocuration: linesplit[11..].to_vec(),
        });
    }
    Ok(medgenpubmed)
}
