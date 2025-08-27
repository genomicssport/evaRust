use crate::clinicvar::clinvarmapper;
use crate::structfile::ClinVarInfo;
use crate::structfile::ClinVarOMIM;
use crate::structfile::NCBIgene;
use crate::structfile::NCBIgeneMerged;
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
pub async fn ncbiannotate(
    pathncbi: &str,
    clinvar: &str,
    medgenomim: &str,
    medgenhpo: &str,
    omimsearch: &str,
) -> Result<Vec<NCBIgeneMerged>, Box<dyn Error>> {
    let fileopen = std::fs::File::open(pathncbi).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let returnvector: Vec<Vec<_>> = fileread
        .lines()
        .filter_map(|line: Result<String, _>| line.ok())
        .par_bridge()
        .map(|x| mapiter(x).unwrap())
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

    let mut finalwritevector: Vec<NCBIgeneMerged> = Vec::new();
    let clinvarmapwrite: Vec<ClinVarOMIM> =
        clinvarmapper(clinvar, medgenomim, medgenhpo, omimsearch).unwrap();
    for i in finaljson.iter() {
        for val in clinvarmapwrite.iter() {
            if omimsearch.parse::<usize>().unwrap() == i.disease.clone() {
                finalwritevector.push(NCBIgeneMerged {
                    ncbigeneid: i.ncbigeneid.clone(),
                    genesymbol: i.genesymbol.clone(),
                    association: i.association.clone(),
                    disease: i.disease.to_string().clone(),
                    chrom: val.chrom.clone(),
                    pos: val.pos.clone(),
                    id: val.id.clone(),
                    refid: val.refid.clone(),
                    altallele: val.altallele.clone(),
                    qual: val.qual.clone(),
                    filter: val.filter.clone(),
                    cui: val.cui.clone(),
                    sdui: val.sdui.clone(),
                    hpostr: val.hpostr.clone(),
                    medgenstr: val.medgenstr.clone(),
                    medgenstrsab: val.medgenstrsab.clone(),
                    omimcui: val.omimcui.clone(),
                    omimnumber: val.mimnumber.clone(),
                    mimname: val.omimname.clone(),
                    relationship: val.relationship.clone(),
                    hpocui: val.hpocui.clone(),
                    hponame: val.hponame.clone(),
                    medgensource: val.medgensource.clone(),
                    medgenname: val.medgenname.clone(),
                    sty: val.sty.clone(),
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
    }
    Ok(finalwritevector)
}

pub fn mapiter(lineread: String) -> std::io::Result<Vec<NCBIgene>> {
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
