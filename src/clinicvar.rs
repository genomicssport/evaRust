use crate::hpoomim::hpoomimmap;
use crate::medgenhpo::medgenhpomap;
use crate::structfile::ClinVar;
use crate::structfile::ClinVarInfo;
use crate::structfile::ClinVarOMIM;
use crate::structfile::HPOOMIM;
use crate::structfile::MedgenHPO;
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
pub async fn clinvarmapper(
    clinvar: &str,
    medgenomim: &str,
    medgenhpo: &str,
    omim: &str,
) -> Result<Vec<ClinVarOMIM>, Box<dyn Error>> {
    let fileopen = std::fs::File::open(clinvar).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let returnvector: Vec<Vec<_>> = fileread
        .lines()
        .filter_map(|line: Result<String, _>| line.ok())
        .par_bridge()
        .map(|x| mapiter(x).unwrap())
        .collect::<Vec<_>>();
    let mut finaljson: Vec<ClinVar> = Vec::new();
    for i in returnvector.iter() {
        for val in i.iter() {
            finaljson.push(ClinVar {
                chrom: val.chrom.clone(),
                pos: val.pos.clone(),
                id: val.id.clone(),
                refid: val.refid.clone(),
                altallele: val.altallele.clone(),
                qual: val.qual.clone(),
                filter: val.filter.clone(),
                info: ClinVarInfo {
                    alleleid: val.info.alleleid.clone(),
                    clndisdb: val.info.clndisdb.clone(),
                    clndn: val.info.clndn.clone(),
                    clnhgvs: val.info.clnhgvs.clone(),
                    clnrevstat: val.info.clnrevstat.clone(),
                    clnsig: val.info.clnsig.clone(),
                    clnsigscv: val.info.clnsigscv.clone(),
                    clnvc: val.info.clnvc.clone(),
                    clnvcso: val.info.clnvcso.clone(),
                    clnvi: val.info.clnvi.clone(),
                    geneinfo: val.info.geneinfo.clone(),
                    mc: val.info.mc.clone(),
                    origin: val.info.mc.clone(),
                },
            });
        }
    }
    let hpomap: Vec<HPOOMIM> = hpoomimmap(medgenomim).unwrap();
    let hpomedgen: Vec<MedgenHPO> = medgenhpomap(medgenhpo).unwrap();
    let mut writeclinvar: Vec<ClinVarOMIM> = Vec::new();
    for i in finaljson.iter() {
        for val in hpomap.iter() {
            for en in hpomedgen.iter() {
                if i.info.clndisdb.contains(omim) {
                    writeclinvar.push(ClinVarOMIM {
                        chrom: i.chrom.clone(),
                        pos: i.pos.clone(),
                        id: i.id.clone(),
                        refid: i.refid.clone(),
                        altallele: i.altallele.clone(),
                        qual: i.qual.clone(),
                        filter: i.filter.clone(),
                        info: ClinVarInfo {
                            alleleid: i.info.alleleid.clone(),
                            clndisdb: i.info.clndisdb.clone(),
                            clndn: i.info.clndn.clone(),
                            clnhgvs: i.info.clnhgvs.clone(),
                            clnrevstat: i.info.clnrevstat.clone(),
                            clnsig: i.info.clnsig.clone(),
                            clnsigscv: i.info.clnsigscv.clone(),
                            clnvc: i.info.clnvc.clone(),
                            clnvcso: i.info.clnvcso.clone(),
                            clnvi: i.info.clnvi.clone(),
                            geneinfo: i.info.geneinfo.clone(),
                            mc: i.info.mc.clone(),
                            origin: i.info.mc.clone(),
                        },
                        cui: en.cui.clone(),
                        sdui: en.sdui.clone(),
                        hpostr: en.hpostr.clone(),
                        medgenstr: en.medgenstr.clone(),
                        medgenstrsab: en.medgenstrsab.clone(),
                        sty: val.sty.clone(),
                        omimcui: val.omimcui.clone(),
                        mimnumber: val.mimnumber.clone(),
                        omimname: val.omimname.clone(),
                        relationship: val.relationship.clone(),
                        hpocui: val.hpocui.clone(),
                        hponame: val.hponame.clone(),
                        medgenname: val.medgenname.clone(),
                        medgensource: val.medgensource.clone(),
                    });
                }
            }
        }
    }
    Ok(writeclinvar)
}

/// a nested json mapper for the clinavr
pub fn mapiter(lineread: String) -> std::io::Result<Vec<ClinVar>> {
    let mut clinvar: Vec<ClinVar> = Vec::new();
    let line = lineread.clone();
    if !line.starts_with("#") {
        let linesplit: Vec<_> = line.split("\t").map(String::from).collect::<Vec<_>>();
        clinvar.push(ClinVar {
            chrom: linesplit[0].to_string().clone(),
            pos: linesplit[1].to_string().clone(),
            id: linesplit[2].to_string().clone(),
            refid: linesplit[3].to_string().clone(),
            altallele: linesplit[4].to_string().clone(),
            qual: linesplit[5].to_string().clone(),
            filter: linesplit[6].to_string().clone(),
            info: ClinVarInfo {
                alleleid: linesplit[7].split(";").collect::<Vec<_>>()[0]
                    .split("=")
                    .collect::<Vec<_>>()[1]
                    .to_string(),
                clndisdb: linesplit[7].split(";").collect::<Vec<_>>()[1]
                    .split("=")
                    .collect::<Vec<_>>()[1]
                    .to_string(),
                clndn: linesplit[7].split(";").collect::<Vec<_>>()[2]
                    .split("=")
                    .collect::<Vec<_>>()[1]
                    .to_string(),
                clnhgvs: linesplit[7].split(";").collect::<Vec<_>>()[3]
                    .split("=")
                    .collect::<Vec<_>>()[1]
                    .to_string(),
                clnrevstat: linesplit[7].split(";").collect::<Vec<_>>()[4]
                    .split("=")
                    .collect::<Vec<_>>()[1]
                    .to_string(),
                clnsig: linesplit[7].split(";").collect::<Vec<_>>()[5]
                    .split("=")
                    .collect::<Vec<_>>()[1]
                    .to_string(),
                clnsigscv: linesplit[7].split(";").collect::<Vec<_>>()[6]
                    .split("=")
                    .collect::<Vec<_>>()[1]
                    .to_string(),
                clnvc: linesplit[7].split(";").collect::<Vec<_>>()[7]
                    .split("=")
                    .collect::<Vec<_>>()[1]
                    .to_string(),
                clnvcso: linesplit[7].split(";").collect::<Vec<_>>()[8]
                    .split("=")
                    .collect::<Vec<_>>()[1]
                    .to_string(),
                clnvi: linesplit[7].split(";").collect::<Vec<_>>()[9]
                    .split("=")
                    .collect::<Vec<_>>()[1]
                    .to_string(),
                geneinfo: linesplit[7].split(";").collect::<Vec<_>>()[10]
                    .split("=")
                    .collect::<Vec<_>>()[1]
                    .to_string(),
                mc: linesplit[7].split(";").collect::<Vec<_>>()[11]
                    .split("=")
                    .collect::<Vec<_>>()[1]
                    .to_string(),
                origin: linesplit[7].split(";").collect::<Vec<_>>()[12]
                    .split("=")
                    .collect::<Vec<_>>()[1]
                    .to_string(),
            },
        });
    }
    Ok(clinvar)
}
