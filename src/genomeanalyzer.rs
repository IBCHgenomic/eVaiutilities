use crate::structfile::Combiner;
use crate::structfile::Genomeanalyzer;
use crate::structfile::Genomecapture;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
/*

 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-3-12

*/

pub fn genomemap(path1: &str, path2: &str) -> Result<String, Box<dyn Error>> {
    let fileopen = File::open(path1).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let mut filesplit: Vec<Genomeanalyzer> = Vec::new();
    let mut hashids: HashSet<String> = HashSet::new();

    for i in fileread.lines() {
        let line = i.expect("line not present");
        if line.starts_with("#") {
            continue;
        }
        if !line.starts_with("#") {
            let linevec = line.split("\t").collect::<Vec<_>>();
            filesplit.push(Genomeanalyzer {
                chrom: linevec[0].to_string(),
                start: linevec[1].to_string(),
                stop: linevec[2].to_string(),
                generef: linevec[3].to_string(),
                alt: linevec[4].to_string(),
                priortranscript: linevec[5].to_string(),
                hgvsp: linevec[6].to_string(),
                hgvpc: linevec[7].to_string(),
                cannonical: linevec[8].to_string(),
                othertranscript: linevec[9].to_string(),
                genotype: linevec[10].to_string(),
                gene: linevec[11].to_string(),
                phenotype: linevec[11].to_string(),
                medgencui: linevec[12].to_string(),
                inheritance: linevec[13].to_string(),
                finalclass: linevec[14].to_string(),
                score_pathogen: linevec[15].to_string(),
                flag: linevec[16].to_string(),
                note: linevec[17].to_string(),
                vcforig: linevec[18].to_string(),
                pvs1: linevec[19].to_string(),
                ps1: linevec[20].to_string(),
                ps2: linevec[21].to_string(),
                ps3: linevec[22].to_string(),
                ps4: linevec[23].to_string(),
                pm1: linevec[23].to_string(),
                pm2: linevec[24].to_string(),
                pm3: linevec[25].to_string(),
                pm4: linevec[26].to_string(),
                pm5: linevec[27].to_string(),
                pm6: linevec[28].to_string(),
                pp1: linevec[29].to_string(),
                pp2: linevec[30].to_string(),
                pp3: linevec[31].to_string(),
                pp4: linevec[32].to_string(),
                pp5: linevec[33].to_string(),
                ba1: linevec[34].to_string(),
                bs1: linevec[35].to_string(),
                bs2: linevec[36].to_string(),
                bs3: linevec[37].to_string(),
                bs4: linevec[38].to_string(),
                bp1: linevec[39].to_string(),
                bp2: linevec[40].to_string(),
                bp3: linevec[41].to_string(),
                bp4: linevec[42].to_string(),
                bp5: linevec[43].to_string(),
                bp6: linevec[44].to_string(),
                bp7: linevec[45].to_string(),
                bp8: linevec[46].to_string(),
            });
            hashids.insert(linevec[5].to_string());
        }
    }

    let fileopen2 = File::open(path2).expect("file not present");
    let fileread2 = BufReader::new(fileopen2);

    let mut genomecapture: Vec<Genomecapture> = Vec::new();
    for i in fileread2.lines() {
        let line = i.expect("line not present");
        if line.starts_with("#") {
            continue;
        } else if !line.starts_with("#") {
            let linevec = line.split("\t").collect::<Vec<_>>();
            genomecapture.push(Genomecapture {
                chr: linevec[0].to_string(),
                start: linevec[1].to_string(),
                end: linevec[2].to_string(),
                generef: linevec[3].to_string(),
                alt: linevec[4].to_string(),
                effect: linevec[5].to_string(),
                gene: linevec[6].to_string(),
                transcript: linevec[7].to_string(),
                selectcannonical: linevec[8].to_string(),
                tfbsid: linevec[9].to_string(),
                tfbsname: linevec[10].to_string(),
                exonintronnum: linevec[11].to_string(),
                hgvsc: linevec[12].to_string(),
                hgvsp: linevec[13].to_string(),
                cdsdistance: linevec[14].to_string(),
                cdslen: linevec[15].to_string(),
                aalen: linevec[16].to_string(),
                othertranscripts: linevec[17].to_string(),
                exac_an: linevec[18].to_string(),
                exac_ac: linevec[19].to_string(),
                exac_af: linevec[20].to_string(),
                exac_istarget: linevec[21].to_string(),
                dnsnp: linevec[22].to_string(),
                dnsnp_version: linevec[23].to_string(),
                dbsnp_1tgp_ref_freq: linevec[24].to_string(),
                dbsnp_1tgp_alt_freq: linevec[25].to_string(),
                common_1tgp_1perc: linevec[26].to_string(),
                esp6500siv2_ea_freq: linevec[27].to_string(),
                esp6500siv2_aa_freq: linevec[28].to_string(),
                esp6500siv2_all_freq: linevec[29].to_string(),
                gnomad_af_all: linevec[30].to_string(),
                gnomad_hom_all: linevec[31].to_string(),
                gnomad_af_max_pop: linevec[32].to_string(),
                cadd_score: linevec[33].to_string(),
                dbscsnv_ab_score: linevec[34].to_string(),
                dbscsnv_rf_score: linevec[35].to_string(),
                papi_pred: linevec[36].to_string(),
                papi_score: linevec[37].to_string(),
                polyphen_2_pred: linevec[38].to_string(),
                polyphen_2_score: linevec[39].to_string(),
                sift_pred: linevec[40].to_string(),
                sift_score: linevec[41].to_string(),
                pseeac_rf_pred: linevec[42].to_string(),
                pseeac_rf_score: linevec[43].to_string(),
                clinvar_hotspot: linevec[44].to_string(),
                clinvar_rcv: linevec[45].to_string(),
                clinvar_clinical_significance: linevec[46].to_string(),
                clinvar_rev_status: linevec[47].to_string(),
                clinvar_traitsclinvar_pmids: linevec[48].to_string(),
                diseases: linevec[49].to_string(),
                disease_ids: linevec[50].to_string(),
                aml_0156_dna131_geno: linevec[51].to_string(),
                aml_0156_dna131_qual: linevec[52].to_string(),
                aml_0156_dna131_geno_qual: linevec[53].to_string(),
                aml_0156_dna131_filter: linevec[54].to_string(),
                aml_0156_dna131_af: linevec[55].to_string(),
                aml_0156_dna131_ao: linevec[56].to_string(),
                aml_0156_dna131_ro: linevec[57].to_string(),
                aml_0156_dna131_co: linevec[58].to_string(),
            })
        }
    }

    let mut combineanalyzer: Vec<Combiner> = Vec::new();

    for hashid in hashids.iter() {
        for j in filesplit.iter() {
            for genomeiter in genomecapture.iter() {
                if* hashid == j.priortranscript && genomeiter.othertranscripts.contains(hashid) {
                    combineanalyzer.push(Combiner {
                        chrom: j.chrom.clone(),
                        start: j.start.clone(),
                        stop: j.stop.clone(),
                        priortranscript: j.priortranscript.clone(),
                        hgvpc: j.hgvpc.clone(),
                        cannonical: j.cannonical.clone(),
                        othertranscript: j.othertranscript.clone(),
                        genotype: j.genotype.clone(),
                        phenotype: j.phenotype.clone(),
                        medgencui: j.medgencui.clone(),
                        inheritance: j.inheritance.clone(),
                        finalclass: j.finalclass.clone(),
                        score_pathogen: j.score_pathogen.clone(),
                        flag: j.flag.clone(),
                        note: j.note.clone(),
                        vcforig: j.vcforig.clone(),
                        pvs1: j.pvs1.clone(),
                        ps1: j.ps1.clone(),
                        ps2: j.ps2.clone(),
                        ps3: j.ps3.clone(),
                        ps4: j.ps4.clone(),
                        pm1: j.pm1.clone(),
                        pm2: j.pm2.clone(),
                        pm3: j.pm3.clone(),
                        pm4: j.pm4.clone(),
                        pm5: j.pm5.clone(),
                        pm6: j.pm6.clone(),
                        pp1: j.pp1.clone(),
                        pp2: j.pp2.clone(),
                        pp3: j.pp3.clone(),
                        pp4: j.pp4.clone(),
                        pp5: j.pp5.clone(),
                        ba1: j.ba1.clone(),
                        bs1: j.bs1.clone(),
                        bs2: j.bs2.clone(),
                        bs3: j.bs3.clone(),
                        bs4: j.bs4.clone(),
                        bp1: j.bp1.clone(),
                        bp2: j.bp2.clone(),
                        bp3: j.bp3.clone(),
                        bp4: j.bp4.clone(),
                        bp5: j.bp5.clone(),
                        bp6: j.bp6.clone(),
                        bp7: j.bp7.clone(),
                        bp8: j.bp8.clone(),
                        effect: genomeiter.effect.clone(),
                        transcript: genomeiter.transcript.clone(),
                        selectcannonical: genomeiter.selectcannonical.clone(),
                        tfbsid: genomeiter.tfbsid.clone(),
                        tfbsname: genomeiter.tfbsname.clone(),
                        exonintronnum: genomeiter.exonintronnum.clone(),
                        hgvsc: genomeiter.hgvsc.clone(),
                        cdsdistance: genomeiter.cdsdistance.clone(),
                        cdslen: genomeiter.cdslen.clone(),
                        aalen: genomeiter.aalen.clone(),
                        othertranscripts: genomeiter.othertranscripts.clone(),
                        exac_an: genomeiter.exac_an.clone(),
                        exac_ac: genomeiter.exac_ac.clone(),
                        exac_af: genomeiter.exac_af.clone(),
                        exac_istarget: genomeiter.exac_istarget.clone(),
                        dnsnp: genomeiter.dnsnp.clone(),
                        dnsnp_version: genomeiter.dnsnp_version.clone(),
                        dbsnp_1tgp_ref_freq: genomeiter.dbsnp_1tgp_ref_freq.clone(),
                        dbsnp_1tgp_alt_freq: genomeiter.dbsnp_1tgp_alt_freq.clone(),
                        common_1tgp_1perc: genomeiter.common_1tgp_1perc.clone(),
                        esp6500siv2_ea_freq: genomeiter.esp6500siv2_ea_freq.clone(),
                        esp6500siv2_aa_freq: genomeiter.esp6500siv2_aa_freq.clone(),
                        esp6500siv2_all_freq: genomeiter.esp6500siv2_all_freq.clone(),
                        gnomad_af_all: genomeiter.gnomad_af_all.clone(),
                        gnomad_hom_all: genomeiter.gnomad_hom_all.clone(),
                        gnomad_af_max_pop: genomeiter.gnomad_af_max_pop.clone(),
                        cadd_score: genomeiter.cadd_score.clone(),
                        dbscsnv_ab_score: genomeiter.dbscsnv_ab_score.clone(),
                        dbscsnv_rf_score: genomeiter.dbscsnv_rf_score.clone(),
                        papi_pred: genomeiter.papi_pred.clone(),
                        papi_score: genomeiter.papi_score.clone(),
                        polyphen_2_pred: genomeiter.polyphen_2_pred.clone(),
                        polyphen_2_score: genomeiter.polyphen_2_score.clone(),
                        sift_pred: genomeiter.sift_pred.clone(),
                        sift_score: genomeiter.sift_score.clone(),
                        pseeac_rf_pred: genomeiter.pseeac_rf_score.clone(),
                        pseeac_rf_score: genomeiter.pseeac_rf_score.clone(),
                        clinvar_hotspot: genomeiter.clinvar_hotspot.clone(),
                        clinvar_rcv: genomeiter.clinvar_rcv.clone(),
                        clinvar_clinical_significance: genomeiter.clinvar_clinical_significance.clone(),
                        clinvar_rev_status: genomeiter.clinvar_rev_status.clone(),
                        clinvar_traitsclinvar_pmids: genomeiter.clinvar_traitsclinvar_pmids.clone(),
                        diseases: genomeiter.diseases.clone(),
                        disease_ids: genomeiter.disease_ids.clone(),
                        aml_0156_dna131_geno: genomeiter.aml_0156_dna131_geno.clone(),
                        aml_0156_dna131_qual: genomeiter.aml_0156_dna131_qual.clone(),
                        aml_0156_dna131_geno_qual: genomeiter.aml_0156_dna131_geno_qual.clone(),
                        aml_0156_dna131_filter: genomeiter.aml_0156_dna131_filter.clone(),
                        aml_0156_dna131_af: genomeiter.aml_0156_dna131_af.clone(),
                        aml_0156_dna131_ao: genomeiter.aml_0156_dna131_ao.clone(),
                        aml_0156_dna131_ro: genomeiter.aml_0156_dna131_ro.clone(),
                        aml_0156_dna131_co: genomeiter.aml_0156_dna131_co.clone(),
                    })
                }
            }
        }
    }
    Ok("The result has been written".to_string())
}
