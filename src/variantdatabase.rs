use crate::structfile::Genomeanalyzer;
use crate::structfile::Genomecapture;
use crate::structfile::VariantCombine;
use rusqlite::{params, Connection, Result};
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
/*

 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-3-14

*/

pub fn variantdatabase(path1: &str, path2: &str) -> Result<String, Box<dyn Error>> {
    let fileopen = File::open(path1).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let binding = path1.to_string();
    let filename = binding.split(".").collect::<Vec<_>>();
    let filenamewrite = format!("{}.{}", filename[0].to_string(), "db");

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
                clinical_trials: linevec[48].to_string(),
                clinvar_traitsclinvar_pmids: linevec[49].to_string(),
                diseases: linevec[50].to_string(),
                disease_ids: linevec[51].to_string(),
                aml_0156_dna131_geno: linevec[52].to_string(),
                aml_0156_dna131_qual: linevec[53].to_string(),
                aml_0156_dna131_geno_qual: linevec[54].to_string(),
                aml_0156_dna131_filter: linevec[55].to_string(),
                aml_0156_dna131_af: linevec[56].to_string(),
                aml_0156_dna131_ao: linevec[57].to_string(),
                aml_0156_dna131_ro: linevec[58].to_string(),
                aml_0156_dna131_co: linevec[59].to_string(),
            })
        }
    }

    let mut combineanalyzer: Vec<VariantCombine> = Vec::new();

    for hashid in hashids.iter() {
        for j in filesplit.iter() {
            for genomeiter in genomecapture.iter() {
                if *hashid == j.priortranscript && genomeiter.othertranscripts.contains(hashid) {
                    combineanalyzer.push(VariantCombine {
                        chrom: j.chrom.clone(),
                        start: j.start.clone(),
                        stop: j.stop.clone(),
                        alt: j.alt.clone(),
                        generef: j.generef.clone(),
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
                        clinvar_clinical_significance: genomeiter
                            .clinvar_clinical_significance
                            .clone(),
                        clinvar_rev_status: genomeiter.clinvar_rev_status.clone(),
                        clinical_trials: genomeiter.clinical_trials.clone(),
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
                    });
                }
            }
        }
    }

    let variantdatabase = Connection::open(filenamewrite)?;
    variantdatabase.execute(
        "create table if no exits variants(
            id integer primary key,
            chrom text not null unique,
            start text not null,
            stop text not null,
            priortranscript text not null,
            hgvpc text not null,
            cannonical text not null,
            othertranscript text not null,
            genotype text not null,
            phenotype text not null,
            medgencui text not null,
            inheritance text not null,
            finalclass text not null,
            score_pathogen text not null,
            flag text not null,
            note text not null,
            vcforig text not null,
            pvs1 text not null,
            ps1 text not null,
            ps2 text not null,
            ps3 text not null,
            ps4 text not null,
            pm1 text not null,
            pm2 text not null,
            pm3 text not null,
            pm4 text not null,
            pm5 text not null,
            pm6 text not null,
            pp1 text not null,
            pp2 text not null,
            pp3 text not null,
            pp4 text not null,
            pp5 text not null,
            ba1 text not null,
            bs1 text not null,
            bs2 text not null,
            bs3 text not null,
            bs4 text not null,
            bp1 text not null,
            bp2 text not null,
            bp3 text not null,
            bp4 text not null,
            bp5 text not null,
            bp6 text not null,
            bp7 text not null,
            bp8 text not null,
            effect text not null,
            transcript text not null,
            selectcannonical text not null,
            tfbsid text not null,
            tfbsname text not null,
            exonintronnum text not null,
            hgvsc text not null,
            cdsdistance text not null,
            cdslen text not null,
            aalen text not null,
            othertranscripts text not null,
            exac_an text not null,
            exac_ac text not null,
            exac_af text not null,
            exac_istarget text not null,
            dnsnp text not null,
            dnsnp_version text not null,
            dbsnp_1tgp_ref_freq text not null,
            dbsnp_1tgp_alt_freq text not null,
            common_1tgp_1perc text not null,
            esp6500siv2_ea_freq text not null,
            esp6500siv2_aa_freq text not null,
            esp6500siv2_all_freq text not null,
            gnomad_af_all text not null,
            gnomad_hom_all text not null,
            gnomad_af_max_pop text not null,
            cadd_score text not null,
            dbscsnv_ab_score text not null,
            dbscsnv_rf_score text not null,
            papi_pred text not null,
            papi_score text not null,
            polyphen_2_pred text not null,
            polyphen_2_score text not null,
            sift_pred text not null,
            sift_score text not null,
            pseeac_rf_pred text not null,
            pseeac_rf_score text not null,
            clinvar_hotspot text not null,
            clinvar_rcv text not null,
            clinvar_clinical_significance text not null,
            clinvar_rev_status text not null,
            clinvar_traitsclinvar_pmids text not null,
            diseases text not null,
            disease_ids text not null,
            aml_0156_dna131_geno text not null,
            aml_0156_dna131_qual text not null,
            aml_0156_dna131_geno_qual text not null,
            aml_0156_dna131_filter text not null,
            aml_0156_dna131_af text not null,
            aml_0156_dna131_ao text not null,
            aml_0156_dna131_ro text not null,
            aml_0156_dna131_co text not null,
        )",
        [],
    )?;

    for i in combineanalyzer.iter() {
        variantdatabase.execute("INSERT INTO variants (chrom, start, stop, priortranscript, hgvpc, cannonical, othertranscript, genotype, phenotype, medgencui, inheritance, finalclass, score_pathogen, flag, note, vcforig, pvs1, ps1, ps2, ps3, ps4, pm1, pm2, pm3, pm4, pm5, pm6, pp1, pp2, pp3, pp4, pp5, ba1, bs1, bs2, bs3, bs4, bp1, bp2, bp3, bp4, bp5, bp6, bp7, bp8, effect, transcript, selectcannonical, tfbsid, tfbsname, exonintronnum, hgvsc, cdsdistance, cdslen, aalen, othertranscripts, exac_an, exac_ac, exac_af, exac_istarget, dbsnp, dbsnp_version, dbsnp_1tgp_ref_freq, dbsnp_1tgp_alt_freq, common_1tgp_1perc,esp6500siv2_ea_freq,esp6500siv2_aa_freq, esp6500siv2_all_freq,gnomad_af_all, gnomad_hom_all,gnomad_af_max_pop, cadd_score,dbscsnv_ab_score, dbscsnv_rf_score, papi_pred, papi_score, polyphen_2_pred,polyphen_2_score, sift_pred,
sift_score, pseeac_rf_pred, pseeac_rf_score, clinvar_hotspot, clinvar_rcv, clinvar_clinical_significance,clinvar_rev_status, clinvar_traitsclinvar_pmids, diseases, disease_ids, aml_0156_dna131_geno, aml_0156_dna131_qual, aml_0156_dna131_geno_qual, aml_0156_dna131_filter, aml_0156_dna131_af, aml_0156_dna131_ao, aml_0156_dna131_ro, aml_0156_dna131_co,
) values (:?1, :?2, :?3, :?4, :?5, :?6, :?7. :?8, :?9, :?10, :?11, :?12, :?13, :?14, :?15, :?16, :?17, :?18, :?19, :?20, :?21, :?22, :?23, :?24, :?25, :?26, :?27, :?28, :?29, :?30, :?31, :?32, :?33, :?34, :?35, :?36, :?37, :?38, :?39, :?40, :?41, :?42, :?43, :?44, :?45, :?46, :?47, :?48, :?49, :?50, :?51, :?52, :?53, :?54, :?55, :?56, :?57, :?58, :?59, :?60, :?61, :?62, :?63, :?64, :?65, :?66, :?67, :?68, :?69, :?70, :?71,:?72, :?73, :?74, :?75, :?76, :?77, :?78, :?79, :?80, :?81, :?82, :?83, :?84, :?85, :?86, :?87, :?88, :?89, :?90. :?91, :?92, :?93, :?94, :?95, :?96, :?97, :?98, :?99, :?100 )", params![i.chrom, i.start, i.stop, i.priortranscript, i.hgvpc, i.cannonical, i.othertranscript, i.genotype, i.phenotype, i.medgencui, i.inheritance, i.finalclass, i.score_pathogen, i.flag, i.note, i.vcforig, i.pvs1, i.ps1, i.ps2, i.ps3, i.ps4, i.pm1, i.pm2, i.pm3, i.pm4, i.pm4, i.pm5, i.pm6, i.pp1, i.pp2, i.pp3, i.pp4, i.pp5, i.ba1, i.bs1, i.bs2, i.bs3, i.bs4, i.bs4, i.bp1, i.bp2, i.bp3, i.bp4, i.bp4, i.bp5, i.bp6, i.bp7, i.bp8, i.effect, i.transcript, i.selectcannonical, i.tfbsid, i.tfbsname, i.exonintronnum, i.hgvsc, i.cdsdistance, i.cdslen, i.aalen, i.othertranscript, i. exac_an, i.exac_ac, i.exac_af, i.exac_istarget, i.dnsnp, i.dnsnp_version, i.dbsnp_1tgp_ref_freq, i.dbsnp_1tgp_alt_freq, i.common_1tgp_1perc,i.esp6500siv2_ea_freq,i.esp6500siv2_aa_freq, i.esp6500siv2_all_freq,i.gnomad_af_all, i.gnomad_hom_all,i.gnomad_af_max_pop, i.cadd_score,i.dbscsnv_ab_score, i.dbscsnv_rf_score, i.papi_pred, i.papi_score, i.polyphen_2_pred,i.polyphen_2_score, i.sift_pred,i.sift_score, i.pseeac_rf_pred, i.pseeac_rf_score, i.clinvar_hotspot, i.clinvar_rcv, i.clinvar_clinical_significance,i.clinvar_rev_status, i.clinvar_traitsclinvar_pmids, i.diseases, i.disease_ids, i.aml_0156_dna131_geno, i.aml_0156_dna131_qual, i.aml_0156_dna131_geno_qual, i.aml_0156_dna131_filter, i.aml_0156_dna131_af, i.aml_0156_dna131_ao, i.aml_0156_dna131_ro, i.aml_0156_dna131_co])?;
    }

    Ok("The result has been written".to_string())
}
