use crate::hpoomim::hpoomimmap;
use crate::medgenhpo::medgenhpomap;
use crate::structfile::ClinVar;
use crate::structfile::ClinVarInfo;
use crate::structfile::ClinVarOMIM;
use crate::structfile::MedgenHPO;
use crate::structfile::VCFAnnotate;
use crate::structfile::HPOOMIM;
use dotenv::dotenv;
use rayon::prelude::*;
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

pub fn clinvarvcf(vcffile: &str, clinvar: &str) -> Result<Vec<ClinVarOMIM>, Box<dyn Error>> {
    /*adding a dotenv for the other files(later will
    add for all the files so that it reads the
    databae automatically)
    */
    dotenv().ok();
    let medgenomim = std::env::var("file1").expect("file not present");
    let medgenhpo = std::env::var("file2").expect("file not present");
    let filevcf = File::open(vcffile).expect("File not present");
    let filevcfread = BufReader::new(filevcf);
    let vcfvector: Vec<Vec<_>> = filevcfread
        .lines()
        .filter_map(|line: Result<String, _>| line.ok())
        .par_bridge()
        .map(|x| vcfread(&x).unwrap())
        .collect::<Vec<_>>();
    let fileopen = std::fs::File::open(clinvar).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let returnvector: Vec<Vec<_>> = fileread
        .lines()
        .filter_map(|line: Result<String, _>| line.ok())
        .par_bridge()
        .map(|x| mapiter(x).unwrap())
        .collect::<Vec<_>>();
    let mut finalclinvar: Vec<ClinVar> = Vec::new();
    for i in returnvector.iter() {
        for val in i.iter() {
            finalclinvar.push(ClinVar {
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

    let mut final_vcf_annotate: Vec<VCFAnnotate> = Vec::new();
    for i in vcfvector.iter() {
        for val in i.iter() {
            final_vcf_annotate.push(VCFAnnotate {
                chromo: val.chromo.clone(),
                position: val.position.clone(),
                id: val.id.clone(),
                refallele: val.refallele.clone(),
                altallele: val.altallele.clone(),
                quality: val.quality.clone(),
            });
        }
    }

    let mut clinvar_vcf_annotate: Vec<ClinVarOMIM> = Vec::new();
    let hpomap: Vec<HPOOMIM> = hpoomimmap(&medgenomim).unwrap();
    let hpomedgen: Vec<MedgenHPO> = medgenhpomap(&medgenhpo).unwrap();
    for i in final_vcf_annotate.iter() {
        for val in hpomap.iter() {
            for en in hpomedgen.iter() {
                for clvar in finalclinvar.iter() {
                    if i.position == clvar.pos.clone().parse::<usize>().unwrap()
                        && i.refallele == clvar.refid.clone()
                        && i.altallele == clvar.altallele
                    {
                        clinvar_vcf_annotate.push(ClinVarOMIM {
                            chrom: clvar.chrom.clone(),
                            pos: clvar.pos.clone(),
                            id: clvar.id.clone(),
                            refid: clvar.refid.clone(),
                            altallele: clvar.altallele.clone(),
                            qual: clvar.qual.clone(),
                            filter: clvar.filter.clone(),
                            info: ClinVarInfo {
                                alleleid: clvar.info.alleleid.clone(),
                                clndisdb: clvar.info.clndisdb.clone(),
                                clndn: clvar.info.clndn.clone(),
                                clnhgvs: clvar.info.clnhgvs.clone(),
                                clnrevstat: clvar.info.clnrevstat.clone(),
                                clnsig: clvar.info.clnsig.clone(),
                                clnsigscv: clvar.info.clnsigscv.clone(),
                                clnvc: clvar.info.clnvc.clone(),
                                clnvcso: clvar.info.clnvcso.clone(),
                                clnvi: clvar.info.clnvi.clone(),
                                geneinfo: clvar.info.geneinfo.clone(),
                                mc: clvar.info.mc.clone(),
                                origin: clvar.info.mc.clone(),
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
    }
    Ok(clinvar_vcf_annotate)
}

pub fn vcfread(line: &str) -> Result<Vec<VCFAnnotate>, Box<dyn Error>> {
    let lineinternal: String = line.to_string();
    let mut vcfannotate: Vec<VCFAnnotate> = Vec::new();
    if !lineinternal.starts_with("#") {
        let line = lineinternal.split("\t").collect::<Vec<_>>();
        vcfannotate.push(VCFAnnotate {
            chromo: line[0].to_string(),
            position: line[1].parse::<usize>().unwrap(),
            id: line[2].to_string().replace("rs", ""),
            refallele: line[3].to_string(),
            altallele: line[4].to_string(),
            quality: line[5].to_string(),
        });
    }
    Ok(vcfannotate)
}

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
