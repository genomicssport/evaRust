use crate::hpoomim::hpoomimmap;
use crate::medgenhpo::medgenhpomap;
use crate::medgenpubmed::medgenpubmedmap;
use crate::readmedgen::medgenmapmap;
use std::error::Error;

/*
Author Gaurav Sablok
Instytut Chemii Bioorganicznej
Polskiej Akademii Nauk
ul. Noskowskiego 12/14 | 61-704, Poznań
Date: 2025-7-23
*/

#[tokio::main]
pub async fn cuiparallel(
    medgenhpo: &str,
    medgen_omim: &str,
    medgenmapping: &str,
    medgenpubmed: &str,
) -> Result<String, Box<dyn Error>> {
    let _finalmedgenfile = medgenhpomap(medgenhpo).unwrap();
    let _finalmgenomim = hpoomimmap(medgen_omim).unwrap();
    let _finalmedgenmapping = medgenmapmap(medgenmapping).unwrap();
    let _finalmedgenpubmed = medgenpubmedmap(medgenpubmed).unwrap();

    Ok("The serialization for the CUI has been written".to_string())
}
