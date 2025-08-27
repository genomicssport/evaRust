use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "eVaRust",
    version = "1.0",
    about = "Variant visualizer for human and mouse genomics.
       ************************************************
      Gaurav Sablok, IBCH, PAN, Poznan, Poland,
      https://portal.ichb.pl/laboratory-of-genomics/.
      Email: gsablok@ibch.poznan.pl
      ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// prepapre the CUI for the medgen
    CUIGENERATE {
        /// medgen HPO file
        medgenhpo: String,
        /// medgen OMIM file
        medgen_omim: String,
        /// medgen mapping
        medgenmapping: String,
        /// medgen pubmed
        medgenpubmed: String,
    },
    /// OMIM and Evidence Annotator
    OMIM {
        /// generate the link to OMIM and NCBI
        omimfile: String,
        /// evidence number
        evidencenumber: String,
        /// HPO mapping
        hpomapping: String,
        /// HPO megdgen file
        hpomedgen: String,
    },
    /// clinicvar OMIM and Evidence annotator
    CLINVAROMIMEVIDENCE {
        /// provide the clinicavar file
        clinvar: String,
        /// provide the medgen file
        medgen: String,
        /// provide the medgenhpo file
        medgenhpo: String,
        /// provide the OMIM number
        omim: String,
    },
    /// NCBI gene annotate
    NCBIANNOTATE {
        /// provide the ncbigene id file
        ncbigeneid: String,
        /// provide the clinvar file
        clinvar: String,
        /// provide the medgenomim file
        medgenomim: String,
        /// provide the medgenhpo file
        medgenhpo: String,
        /// provide the OMIM number
        omimsearch: String,
    },
    /// Multistage annotation linker
    ANNOTATOR {
        ///maxo annotations file
        pathncbimaxo: String,
        /// provide the medgenomim file
        medgenomim: String,
        /// provide the medgenhpo file
        medgenhpo: String,
        /// provide the evidence number
        evidence: String,
    },
    /// annotate vcf to clinvar and medgen
    VCFCLINVARANNOTATE {
        /// provide the vcf file for annotation
        vcffile: String,
        /// provide the clinvar file for annotation
        clinvar: String,
    },
    /// Phenotype associations
    PHENOTYPELINKER {
        /// genes disease association
        genesdisease: String,
        /// genes phenotype association
        genesphenotype: String,
        /// phenotype hpoa
        phenotypehpoa: String,
        /// phenotypes to genes
        phenotypesgenes: String,
    },
    /// Download databases
    Databases {
        /// download the databases for the annotation
        databaseoption: Option<bool>,
    },
}
