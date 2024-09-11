use std::collections::HashMap;

pub fn validate_gc_id(gc_id: usize) -> bool {
    match gc_id {
        1..=6 | 9..=16 | 21..=33 => true,
        _ => false,
    }
}

pub fn get_gen_code_name(gc_id: usize) -> String {
    match validate_gc_id(gc_id) {
        true => GENETIC_CODE_NAMES[gc_id].to_string(),
        false => "NA".to_string(),
    }
}

// 01: Standard
// 02: Vertebrate Mitochondrial
// 03: Yeast Mitochondrial
// 04: Mold Mitochondrial; Protozoan Mitochondrial; Coelenterate Mitochondrial; Mycoplasma; Spiroplasma
// 05: Invertebrate Mitochondrial
// 06: Ciliate Nuclear; Dasycladacean Nuclear; Hexamita Nuclear
// 07: NA
// 08: NA
// 09: Echinoderm Mitochondrial; Flatworm Mitochondrial
// 10: Euplotid Nuclear
// 11: Bacterial, Archaeal and Plant Plastid
// 12: Alternative Yeast Nuclear
// 13: Ascidian Mitochondrial
// 14: Alternative Flatworm Mitochondrial
// 15: Blepharisma Macronuclear
// 16: Chlorophycean Mitochondrial
// 17: NA
// 18: NA
// 19: NA
// 20: NA
// 21: Trematode Mitochondrial
// 22: Scenedesmus obliquus mitochondrial
// 23: Thraustochytrium mitochondrial code
// 24: Pterobranchia Mitochondrial
// 25: Candidate Division SR1 and Gracilibacteria
// 26: Pachysolen tannophilus Nuclear
// 27: Karyorelict Nuclear
// 28: Condylostoma Nuclear
// 29: Mesodinium Nuclear
// 30: Peritrich Nuclear
// 31: Blastocrithidia Nuclear
// 32: Balanophoraceae Plastid
// 33: Cephalodiscidae Mitochondrial

pub const GENETIC_CODE_NAMES: [&str; 34] = [
  "NA",
  "Standard",
  "Vertebrate Mitochondrial",
  "Yeast Mitochondrial",
  "Mold Mitochondrial; Protozoan Mitochondrial; Coelenterate Mitochondrial; Mycoplasma; Spiroplasma",
  "Invertebrate Mitochondrial",
  "Ciliate Nuclear; Dasycladacean Nuclear; Hexamita Nuclear",
  "NA",
  "NA",
  "Echinoderm Mitochondrial; Flatworm Mitochondrial",
  "Euplotid Nuclear",
  "Bacterial, Archaeal and Plant Plastid",
  "Alternative Yeast Nuclear",
  "Ascidian Mitochondrial",
  "Alternative Flatworm Mitochondrial",
  "Blepharisma Macronuclear",
  "Chlorophycean Mitochondrial",
  "NA",
  "NA",
  "NA",
  "NA",
  "Trematode Mitochondrial",
  "Scenedesmus obliquus mitochondrial",
  "Thraustochytrium mitochondrial code",
  "Pterobranchia Mitochondrial",
  "Candidate Division SR1 and Gracilibacteria",
  "Pachysolen tannophilus Nuclear",
  "Karyorelict Nuclear",
  "Condylostoma Nuclear",
  "Mesodinium Nuclear",
  "Peritrich Nuclear",
  "Blastocrithidia Nuclear",
  "Balanophoraceae Plastid",
  "Cephalodiscidae Mitochondrial",
];

pub fn get_trans_table(gc_id: usize) -> HashMap<String, String> {
    match validate_gc_id(gc_id) {
        true => {
            let codons: [&str; 64] = CODONS;
            let aas: &str = GENETIC_CODE_AMINO_ACIDS[gc_id];
            let mut tt: HashMap<String, String> = HashMap::with_capacity(64);
            for (i, codon) in codons.iter().enumerate() {
                let aa = aas.get(i..=i).unwrap();
                tt.insert(codon.to_string(), aa.to_string());
            }
            return tt;
        }
        false => todo!(),
    }
}

pub const GENETIC_CODE_AMINO_ACIDS: [&str; 34] = [
    "****************************************************************",
    "FFLLSSSSYY**CC*WLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 01
    "FFLLSSSSYY**CCWWLLLLPPPPHHQQRRRRIIMMTTTTNNKKSS**VVVVAAAADDEEGGGG", // 02
    "FFLLSSSSYY**CCWWTTTTPPPPHHQQRRRRIIMMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 03
    "FFLLSSSSYY**CCWWLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 04
    "FFLLSSSSYY**CCWWLLLLPPPPHHQQRRRRIIMMTTTTNNKKSSSSVVVVAAAADDEEGGGG", // 05
    "FFLLSSSSYYQQCC*WLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 06
    "****************************************************************",
    "****************************************************************",
    "FFLLSSSSYY**CCWWLLLLPPPPHHQQRRRRIIIMTTTTNNNKSSSSVVVVAAAADDEEGGGG", // 09
    "FFLLSSSSYY**CCCWLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 10
    "FFLLSSSSYY**CC*WLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 11
    "FFLLSSSSYY**CC*WLLLSPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 12
    "FFLLSSSSYY**CCWWLLLLPPPPHHQQRRRRIIMMTTTTNNKKSSGGVVVVAAAADDEEGGGG", // 13
    "FFLLSSSSYYY*CCWWLLLLPPPPHHQQRRRRIIIMTTTTNNNKSSSSVVVVAAAADDEEGGGG", // 14
    "FFLLSSSSYY*QCC*WLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 15
    "FFLLSSSSYY*LCC*WLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 16
    "****************************************************************",
    "****************************************************************",
    "****************************************************************",
    "****************************************************************",
    "FFLLSSSSYY**CCWWLLLLPPPPHHQQRRRRIIMMTTTTNNNKSSSSVVVVAAAADDEEGGGG", // 21
    "FFLLSS*SYY*LCC*WLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 22
    "FF*LSSSSYY**CC*WLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 23
    "FFLLSSSSYY**CCWWLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSSKVVVVAAAADDEEGGGG", // 24
    "FFLLSSSSYY**CCGWLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 25
    "FFLLSSSSYY**CC*WLLLAPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 26
    "FFLLSSSSYYQQCCWWLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 27
    "FFLLSSSSYYQQCCWWLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 28
    "FFLLSSSSYYYYCC*WLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 29
    "FFLLSSSSYYEECC*WLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 30
    "FFLLSSSSYYEECCWWLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 31
    "FFLLSSSSYY*WCC*WLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 32
    "FFLLSSSSYYY*CCWWLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSSKVVVVAAAADDEEGGGG", // 33
];

fn get_start_or_stop_codons(gc_id: usize, ch_match: char) -> Vec<String> {
    match validate_gc_id(gc_id) {
        true => {
            let codons: [&str; 64] = CODONS;
            let sss: &str = GENETIC_CODE_START_STOP[gc_id];
            let mut start_codons: Vec<String> = Vec::new();
            for (i, ch) in sss.chars().enumerate() {
                if ch == ch_match {
                    start_codons.push(codons.get(i).unwrap().to_string())
                } else {
                    continue;
                }
            }
            return start_codons;
        }
        false => todo!(),
    }
}

pub fn get_start_codons(gc_id: usize) -> Vec<String> {
    get_start_or_stop_codons(gc_id, 'M')
}

pub fn get_stop_codons(gc_id: usize) -> Vec<String> {
    get_start_or_stop_codons(gc_id, '*')
}

pub const GENETIC_CODE_START_STOP: [&str; 34] = [
    "****************************************************************",
    "---M------**--*----M---------------M----------------------------", // 01
    "----------**--------------------MMMM----------**---M------------", // 02
    "----------**----------------------MM---------------M------------", // 03
    "--MM------**-------M------------MMMM---------------M------------", // 04
    "---M------**--------------------MMMM---------------M------------", // 05
    "--------------*--------------------M----------------------------", // 06
    "****************************************************************",
    "****************************************************************",
    "----------**-----------------------M---------------M------------", // 09
    "----------**-----------------------M----------------------------", // 10
    "---M------**--*----M------------MMMM---------------M------------", // 11
    "----------**--*----M---------------M----------------------------", // 12
    "---M------**----------------------MM---------------M------------", // 13
    "-----------*-----------------------M----------------------------", // 14
    "----------*---*--------------------M----------------------------", // 15
    "----------*---*--------------------M----------------------------", // 16
    "****************************************************************",
    "****************************************************************",
    "****************************************************************",
    "****************************************************************",
    "----------**-----------------------M---------------M------------", // 21
    "------*---*---*--------------------M----------------------------", // 22
    "--*-------**--*-----------------M--M---------------M------------", // 23
    "---M------**-------M---------------M---------------M------------", // 24
    "---M------**-----------------------M---------------M------------", // 25
    "----------**--*----M---------------M----------------------------", // 26
    "--------------*--------------------M----------------------------", // 27
    "----------**--*--------------------M----------------------------", // 28
    "--------------*--------------------M----------------------------", // 29
    "--------------*--------------------M----------------------------", // 30
    "----------**-----------------------M----------------------------", // 31
    "---M------*---*----M------------MMMM---------------M------------", // 32
    "---M-------*-------M---------------M---------------M------------", // 33
];

pub const CODONS: [&str; 64] = [
    "TTT", "TTC", "TTA", "TTG", "TCT", "TCC", "TCA", "TCG", "TAT", "TAC", "TAA", "TAG", "TGT",
    "TGC", "TGA", "TGG", "CTT", "CTC", "CTA", "CTG", "CCT", "CCC", "CCA", "CCG", "CAT", "CAC",
    "CAA", "CAG", "CGT", "CGC", "CGA", "CGG", "ATT", "ATC", "ATA", "ATG", "ACT", "ACC", "ACA",
    "ACG", "AAT", "AAC", "AAA", "AAG", "AGT", "AGC", "AGA", "AGG", "GTT", "GTC", "GTA", "GTG",
    "GCT", "GCC", "GCA", "GCG", "GAT", "GAC", "GAA", "GAG", "GGT", "GGC", "GGA", "GGG",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_genetic_code_raw_data() {
        assert_eq!(GENETIC_CODE_NAMES.len(), GENETIC_CODE_AMINO_ACIDS.len());
        assert_eq!(GENETIC_CODE_NAMES.len(), GENETIC_CODE_START_STOP.len());
    }

    #[test]
    fn t_get_gen_code_name() {
        assert_eq!(get_gen_code_name(0), "NA");
        assert_eq!(get_gen_code_name(1), "Standard");
    }
}
