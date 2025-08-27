use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::*;

/*
Author Gaurav Sablok
Instytut Chemii Bioorganicznej
Polskiej Akademii Nauk
ul. Noskowskiego 12/14 | 61-704, Poznań
Date: 2025-7-23
*/

#[tokio::main]
pub async fn databasedownload(optionread: Option<bool>) -> Result<String, Box<dyn Error>> {
    if optionread.unwrap() == true {
        let _ = fs::create_dir("./download").unwrap();
        let newpath = Path::new("./download");
        let _ = env::set_current_dir(newpath);
        let _ = Command::new("wget")
            .arg("-F")
            .arg("https://ftp.ncbi.nlm.nih.gov/pub/clinvar/vcf_GRCh38/clinvar.vcf.gz")
            .output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("-F")
            .arg("https://github.com/obophenotype/human-phenotype-ontology/releases/download/v2025-03-03/genes_to_disease.txt")
            .output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("-F")
            .arg("https://github.com/obophenotype/human-phenotype-ontology/releases/download/v2025-03-03/genes_to_phenotype.txt")
            .output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("-F")
            .arg("https://github.com/obophenotype/human-phenotype-ontology/releases/download/v2025-03-03/phenotype.hpoa")
            .output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("-F")
            .arg("https://github.com/obophenotype/human-phenotype-ontology/releases/download/v2025-03-03/phenotype_to_genes.txt")
            .output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("-F")
            .arg("https://ftp.ncbi.nlm.nih.gov/pub/medgen/medgen_pubmed_lnk.txt.gz")
            .output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("-F")
            .arg("https://ftp.ncbi.nlm.nih.gov/pub/medgen/MedGen_HPO_Mapping.txt.gz")
            .output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("-F")
            .arg("https://ftp.ncbi.nlm.nih.gov/pub/medgen/MedGen_HPO_OMIM_Mapping.txt.gz")
            .output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("-F")
            .arg("https://ftp.ncbi.nlm.nih.gov/pub/medgen/MedGenIDMappings.txt.gz")
            .output()
            .expect("command to fail");
    }
    Ok("file for the database has been downloaded".to_string())
}
