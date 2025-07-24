/*
 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-7-23
 */

#[derive(Debug, Clone, PartialOrd, PartialEq)]

pub struct CUIJSON {
    pub sdui: String,
    pub hpostr: String,
    pub medgenstr: String,
    pub medgenstr_sab: String,
    pub sty: String,
    pub relationship: String,
    pub hpocui: String,
    pub mimnumber: String,
    pub omimnumber: String,
    pub hpoid: String,
    pub medgenname: String,
    pub medgensource: String,
    pub prefname: String,
    pub sourceid: String,
    pub source: String,
    pub uid: String,
    pub name: String,
    pub pmid: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct MedgenHPO {
    pub cui: String,
    pub sdui: String,
    pub hpostr: String,
    pub medgenstr: String,
    pub medgenstrsab: String,
    pub sty: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct HPOOMIM {
    pub omimcui: String,
    pub mimnumber: String,
    pub omimname: String,
    pub relationship: String,
    pub hpocui: String,
    pub hpoid: String,
    pub hponame: String,
    pub medgenname: String,
    pub medgensource: String,
    pub sty: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct MedgenMap {
    pub cuiid: String,
    pub prefname: String,
    pub sourceid: String,
    pub source: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct MedgenPubMed {
    pub uid: String,
    pub cui: String,
    pub name: String,
    pub pmid: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct OMIMEvidence {
    pub databaseid: String,
    pub diseasename: String,
    pub qualifier: String,
    pub hpoid: String,
    pub reference: String,
    pub evidence: String,
    pub onset: String,
    pub frequency: String,
    pub sex: String,
    pub modifier: String,
    pub aspect: String,
    pub biocuration: Vec<String>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct OMIMEvidenceMerged {
    pub databaseid: String,
    pub diseasename: String,
    pub qualifier: String,
    pub hpoid: String,
    pub reference: String,
    pub evidence: String,
    pub onset: String,
    pub frequency: String,
    pub sex: String,
    pub modifier: String,
    pub aspect: String,
    pub biocuration: Vec<String>,
    pub cui: String,
    pub sdui: String,
    pub hpostr: String,
    pub medgenstr: String,
    pub medgenstrsab: String,
    pub omimcui: String,
    pub mimnumber: String,
    pub omimname: String,
    pub relationship: String,
    pub hpocui: String,
    pub hponame: String,
    pub medgenname: String,
    pub medgensource: String,
    pub sty: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct ClinVarInfo {
    pub alleleid: String,
    pub clndisdb: String,
    pub clndn: String,
    pub clnhgvs: String,
    pub clnrevstat: String,
    pub clnsig: String,
    pub clnsigscv: String,
    pub clnvc: String,
    pub clnvcso: String,
    pub clnvi: String,
    pub geneinfo: String,
    pub mc: String,
    pub origin: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct ClinVar {
    pub chrom: String,
    pub pos: String,
    pub id: String,
    pub refid: String,
    pub altallele: String,
    pub qual: String,
    pub filter: String,
    pub info: ClinVarInfo,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct ClinVarOMIM {
    pub chrom: String,
    pub pos: String,
    pub id: String,
    pub refid: String,
    pub altallele: String,
    pub qual: String,
    pub filter: String,
    pub info: ClinVarInfo,
    pub cui: String,
    pub sdui: String,
    pub hpostr: String,
    pub medgenstr: String,
    pub medgenstrsab: String,
    pub omimcui: String,
    pub mimnumber: String,
    pub omimname: String,
    pub relationship: String,
    pub hpocui: String,
    pub hponame: String,
    pub medgenname: String,
    pub medgensource: String,
    pub sty: String,
}
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct NCBIgene {
    pub ncbigeneid: String,
    pub genesymbol: String,
    pub association: String,
    pub disease: usize,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct NCBIgeneMerged {
    pub ncbigeneid: String,
    pub genesymbol: String,
    pub association: String,
    pub disease: String,
    pub chrom: String,
    pub pos: String,
    pub id: String,
    pub refid: String,
    pub altallele: String,
    pub qual: String,
    pub filter: String,
    pub info: ClinVarInfo,
    pub cui: String,
    pub sdui: String,
    pub hpostr: String,
    pub medgenstr: String,
    pub medgenstrsab: String,
    pub omimcui: String,
    pub omimnumber: String,
    pub mimname: String,
    pub relationship: String,
    pub hpocui: String,
    pub hponame: String,
    pub medgenname: String,
    pub medgensource: String,
    pub sty: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Ontology {
    pub diseaseid: String,
    pub diseasename: String,
    pub sourceid: String,
    pub maxoid: String,
    pub maxoname: String,
    pub hpoid: usize,
    pub relation: String,
    pub evidence: String,
    pub extensionid: String,
    pub extensionname: String,
    pub comment: String,
    pub other: String,
    pub author: String,
    pub lastupdated: String,
    pub created: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct MergedOntology {
    pub diseaseid: String,
    pub diseasename: String,
    pub sourceid: String,
    pub maxoid: String,
    pub maxoname: String,
    pub hpoids: usize,
    pub relation: String,
    pub evidence: String,
    pub extensionid: String,
    pub extensionname: String,
    pub comment: String,
    pub other: String,
    pub author: String,
    pub lastupdated: String,
    pub created: String,
    pub ncbigeneid: String,
    pub genesymbol: String,
    pub association: String,
    pub disease: String,
    pub cui: String,
    pub sdui: String,
    pub hpostr: String,
    pub medgenstr: String,
    pub medgenstrsab: String,
    pub omimcui: String,
    pub mimnumber: String,
    pub omimname: String,
    pub relationship: String,
    pub hpocui: String,
    pub hpoid: String,
    pub hponame: String,
    pub medgenname: String,
    pub medgensource: String,
    pub sty: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VCFAnnotate {
    pub chromo: String,
    pub position: usize,
    pub id: String,
    pub refallele: String,
    pub altallele: String,
    pub quality: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Phenotype {
    pub hponameid: usize,
    pub hponame: String,
    pub ncbi_geneid: String,
    pub genesymbol: String,
    pub diseaseidpheno: usize,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct PhenotypeHPOA {
    pub databaseid: String,
    pub diseasename: String,
    pub qualifier: String,
    pub hpoid: usize,
    pub reference: String,
    pub evidence: String,
    pub onset: String,
    pub frequency: String,
    pub sex: String,
    pub modifier: String,
    pub aspect: String,
    pub biocuration: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct GenePhenotype {
    pub ncbigeneid: String,
    pub genesymbol: String,
    pub hpoid: usize,
    pub hponame: String,
    pub frequency: String,
    pub diseaseid: usize,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct PhenotypeMerged {
    pub hponameid: usize,
    pub ncbi_geneid: String,
    pub diseaseidpheno: usize,
    pub databaseid: String,
    pub diseasename: String,
    pub qualifier: String,
    pub reference: String,
    pub evidence: String,
    pub onset: String,
    pub sex: String,
    pub modifier: String,
    pub aspect: String,
    pub biocuration: String,
    pub ncbigeneid: String,
    pub genesymbol: String,
    pub hpoid: usize,
    pub hponame: String,
    pub frequency: String,
    pub diseaseid: usize,
    pub association: String,
    pub disease: usize,
}
