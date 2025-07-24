use crate::hpoomim::hpoomimmap;
use crate::medgenhpo::medgenhpomap;
use crate::structfile::MedgenHPO;
use crate::structfile::MergedOntology;
use crate::structfile::NCBIgene;
use crate::structfile::Ontology;
use crate::structfile::HPOOMIM;
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

pub fn ontologyannotate(
    pathncbi: &str,
    medgenomim: &str,
    medgenhpo: &str,
    evidence: &str,
) -> Result<Vec<MergedOntology>, Box<dyn Error>> {
    let fileopen = std::fs::File::open(pathncbi).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let returnvector: Vec<Vec<_>> = fileread
        .lines()
        .filter_map(|line: Result<String, _>| line.ok())
        .par_bridge()
        .map(|x| mapiter(x).unwrap())
        .collect::<Vec<_>>();
    let mut finaloutlay: Vec<Ontology> = Vec::new();
    for i in returnvector.iter() {
        for val in i.iter() {
            finaloutlay.push(Ontology {
                diseaseid: val.diseaseid.clone(),
                diseasename: val.diseasename.clone(),
                sourceid: val.sourceid.clone(),
                maxoid: val.maxoid.clone(),
                maxoname: val.maxoname.clone(),
                hpoid: val.hpoid.clone(),
                relation: val.relation.clone(),
                evidence: val.evidence.clone(),
                extensionid: val.extensionid.clone(),
                extensionname: val.extensionname.clone(),
                comment: val.comment.clone(),
                other: val.other.clone(),
                author: val.author.clone(),
                lastupdated: val.lastupdated.clone(),
                created: val.created.clone(),
            });
        }
    }

    let ncbirecall: Vec<NCBIgene> = ncbigene(pathncbi).unwrap();
    let hpomap: Vec<HPOOMIM> = hpoomimmap(medgenomim).unwrap();
    let hpomedgen: Vec<MedgenHPO> = medgenhpomap(medgenhpo).unwrap();
    let mut valvector: Vec<MergedOntology> = Vec::new();
    for i in finaloutlay.iter() {
        for ncbi in ncbirecall.iter() {
            for hpo in hpomap.iter() {
                for medgen in hpomedgen.iter() {
                    if i.hpoid == evidence.parse::<usize>().unwrap() {
                        valvector.push(MergedOntology {
                            diseaseid: i.diseaseid.clone(),
                            diseasename: i.diseasename.clone(),
                            sourceid: i.sourceid.clone(),
                            maxoid: i.maxoid.clone(),
                            maxoname: i.maxoname.clone(),
                            hpoids: i.hpoid.clone(),
                            relation: i.relation.clone(),
                            evidence: i.evidence.clone(),
                            extensionid: i.extensionid.clone(),
                            extensionname: i.extensionname.clone(),
                            comment: i.comment.clone(),
                            other: i.other.clone(),
                            author: i.author.clone(),
                            lastupdated: i.lastupdated.clone(),
                            created: i.created.clone(),
                            ncbigeneid: ncbi.ncbigeneid.clone(),
                            genesymbol: ncbi.genesymbol.clone(),
                            association: ncbi.association.clone(),
                            disease: ncbi.disease.clone().to_string(),
                            cui: medgen.cui.clone(),
                            sdui: medgen.sdui.clone(),
                            hpostr: medgen.hpostr.clone(),
                            medgenstr: medgen.medgenstr.clone(),
                            medgenstrsab: medgen.medgenstrsab.clone(),
                            sty: medgen.sty.clone(),
                            omimcui: hpo.omimcui.clone(),
                            mimnumber: hpo.mimnumber.clone(),
                            omimname: hpo.omimname.clone(),
                            relationship: hpo.relationship.clone(),
                            hpocui: hpo.hpocui.clone(),
                            hpoid: hpo.hpoid.clone(),
                            hponame: hpo.hponame.clone(),
                            medgenname: hpo.medgenname.clone(),
                            medgensource: hpo.medgensource.clone(),
                        })
                    }
                }
            }
        }
    }
    Ok(valvector)
}

pub fn mapiter(lineread: String) -> std::io::Result<Vec<Ontology>> {
    let mut ontologyvec: Vec<Ontology> = Vec::new();
    let line = lineread.clone();
    if !line.starts_with("#") {
        let linesplit: Vec<_> = line.split("\t").map(String::from).collect::<Vec<_>>();
        ontologyvec.push(Ontology {
            diseaseid: linesplit[0].to_string(),
            diseasename: linesplit[1].to_string(),
            sourceid: linesplit[2].to_string(),
            maxoid: linesplit[3].to_string(),
            maxoname: linesplit[4].to_string(),
            hpoid: linesplit[5].split(":").collect::<Vec<_>>()[1]
                .parse::<usize>()
                .unwrap(),
            relation: linesplit[6].to_string(),
            evidence: linesplit[7].to_string(),
            extensionid: linesplit[8].to_string(),
            extensionname: linesplit[9].to_string(),
            comment: linesplit[10].to_string(),
            other: linesplit[11].to_string(),
            author: linesplit[12].to_string(),
            lastupdated: linesplit[13].to_string(),
            created: linesplit[14].to_string(),
        });
    }
    Ok(ontologyvec)
}

pub fn ncbigene(pathncbi: &str) -> Result<Vec<NCBIgene>, Box<dyn Error>> {
    let fileopen = std::fs::File::open(pathncbi).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let returnvector: Vec<Vec<_>> = fileread
        .lines()
        .filter_map(|line: Result<String, _>| line.ok())
        .par_bridge()
        .map(|x| mapiterncbigene(x).unwrap())
        .collect::<Vec<_>>();
    let mut finaljson: Vec<NCBIgene> = Vec::new();
    for i in returnvector.iter() {
        for val in i.iter() {
            finaljson.push(NCBIgene {
                ncbigeneid: val.ncbigeneid.clone(),
                genesymbol: val.genesymbol.clone(),
                association: val.association.clone(),
                disease: val.disease.clone(),
            });
        }
    }

    Ok(finaljson)
}

pub fn mapiterncbigene(lineread: String) -> std::io::Result<Vec<NCBIgene>> {
    let mut ncbigene: Vec<NCBIgene> = Vec::new();
    let line = lineread.clone();
    if !line.starts_with("#") {
        let linesplit: Vec<_> = line.split("\t").map(String::from).collect::<Vec<_>>();
        ncbigene.push(NCBIgene {
            ncbigeneid: linesplit[0].to_string(),
            genesymbol: linesplit[1].to_string(),
            association: linesplit[2].to_string(),
            disease: linesplit[3].to_string().split(":").collect::<Vec<_>>()[1]
                .parse::<usize>()
                .unwrap(),
        });
    }
    Ok(ncbigene)
}
