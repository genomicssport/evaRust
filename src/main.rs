mod annotation;
mod args;
mod clinicvar;
mod clinvarlinker;
mod databases;
mod hpoomim;
mod medgen;
mod medgenhpo;
mod medgenpubmed;
mod ncbigeneid;
mod omim;
mod phenotype;
mod readmedgen;
mod structfile;
mod unifiedannotator;
use crate::annotation::ontologyannotate;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::clinicvar::clinvarmapper;
use crate::clinvarlinker::clinvarvcf;
use crate::databases::databasedownload;
use crate::medgen::cuiparallel;
use crate::ncbigeneid::ncbiannotate;
use crate::omim::omimevidence;
use crate::phenotype::phenotypeall;
use clap::Parser;
use figlet_rs::FIGfont;

/*
Author Gaurav Sablok
Instytut Chemii Bioorganicznej
Polskiej Akademii Nauk
ul. Noskowskiego 12/14 | 61-704, Poznań
Date: 2025-7-23
*/

fn main() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("eVARUST");
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
    let argsparse = CommandParse::parse();
    match &argsparse.command {
        Commands::CUIGENERATE {
            medgenhpo,
            medgen_omim,
            medgenmapping,
            medgenpubmed,
        } => {
            let command = cuiparallel(medgenhpo, medgen_omim, medgenmapping, medgenpubmed).unwrap();
            println!("The command has been completed:{:?}", command);
        }
        Commands::OMIM {
            omimfile,
            evidencenumber,
            hpomapping,
            hpomedgen,
        } => {
            let command = omimevidence(omimfile, evidencenumber, hpomapping, hpomedgen).unwrap();
            println!("The links for the given evidence are:{:?}", command);
        }
        Commands::CLINVAROMIMEVIDENCE {
            clinvar,
            medgen,
            medgenhpo,
            omim,
        } => {
            let command = clinvarmapper(clinvar, medgen, medgenhpo, omim).unwrap();
            println!("The command has completed:{:?}", command);
        }
        Commands::NCBIANNOTATE {
            ncbigeneid,
            clinvar,
            medgenomim,
            medgenhpo,
            omimsearch,
        } => {
            let command =
                ncbiannotate(ncbigeneid, clinvar, medgenomim, medgenhpo, omimsearch).unwrap();
            println!("The command has completed:{:?}", command);
        }
        Commands::ANNOTATOR {
            pathncbimaxo,
            medgenomim,
            medgenhpo,
            evidence,
        } => {
            let command = ontologyannotate(pathncbimaxo, medgenomim, medgenhpo, evidence).unwrap();
            println!("The command has been completed:{:?}", command)
        }
        Commands::VCFCLINVARANNOTATE { vcffile, clinvar } => {
            let command = clinvarvcf(vcffile, clinvar).unwrap();
            println!(
                "The command has finished and the annotated vcf file has been written:{:?}",
                command
            );
        }
        Commands::PHENOTYPELINKER {
            genesdisease,
            genesphenotype,
            phenotypehpoa,
            phenotypesgenes,
        } => {
            let command =
                phenotypeall(genesdisease, genesphenotype, phenotypehpoa, phenotypesgenes).unwrap();
            println!("The command has finished: {:?}", command);
        }
        Commands::Databases { databaseoption } => {
            let command = databasedownload(*databaseoption).unwrap();
            println!(
                "The command has been finished and all the files have been downloaded:{}",
                command
            );
        }
    }
}
