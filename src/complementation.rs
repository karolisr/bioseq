use std::collections::HashMap;
// use std::iter::zip;

const COMPL_ARR: [(u8, u8); 17] = [
    (45, 45),
    (65, 84),
    (66, 86),
    (67, 71),
    (68, 72),
    (71, 67),
    (72, 68),
    (75, 77),
    (77, 75),
    (78, 78),
    (82, 89),
    (83, 83),
    (84, 65),
    (85, 65),
    (86, 66),
    (87, 87),
    (89, 82),
];

/// Complements a sequence of ASCII bytes (treated as nucleotide (DNA/RNA)).
///
/// Returns a vector of ASCII bytes (DNA).
pub fn complement_bytes(nt: impl Into<Vec<u8>>) -> Vec<u8> {
    // let compl_1: [u8; 17] = *b"ACGTURYMKWSBDHVN-";
    // let compl_2: [u8; 17] = *b"TGCAAYRKMWSVHDBN-";
    // let compl_vec: Vec<(u8, u8)> = zip(compl_1, compl_2).collect();
    // let compl_arr: [(u8, u8); 17] = compl_vec.try_into().unwrap();
    let compl_arr = COMPL_ARR;
    let compl: HashMap<u8, u8> = HashMap::from(compl_arr);
    // dbg!(&compl);
    let mut rv: Vec<u8> = Vec::new();
    let nt: Vec<u8> = nt.into();
    for s in nt {
        rv.push(*compl.get(&s).unwrap_or(&('N' as u8)));
    }
    return rv;
}

/// Complements nucleotide (DNA/RNA) string.
///
/// Returns DNA string.
pub fn complement(nt: impl Into<String>) -> String {
    let nt: String = nt.into().to_ascii_uppercase();
    return String::from_utf8(complement_bytes(nt)).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_complement_bytes() {
        assert_eq!(
            complement_bytes("ACGTURYMKWSBDHVN"),
            [84, 71, 67, 65, 65, 89, 82, 75, 77, 87, 83, 86, 72, 68, 66, 78]
        );
        assert_eq!(complement_bytes([87]), [87]);
        assert_eq!(complement_bytes([78]), [78]);
        assert_eq!(complement_bytes([83]), [83]);
        assert_eq!(complement_bytes([45]), [45]);
        assert_eq!(complement_bytes(b"ACGTURYMKWSBDHVN"), b"TGCAAYRKMWSVHDBN");
    }

    #[test]
    fn t_complement_char() {
        assert_eq!(complement('A'), "T");
        assert_eq!(complement('c'), "G");
        assert_eq!(complement('G'), "C");
        assert_eq!(complement('t'), "A");
        assert_eq!(complement('-'), "-");
    }

    #[test]
    fn t_complement_string() {
        assert_eq!(complement("ACGTURYMKWSBDHVN-"), "TGCAAYRKMWSVHDBN-");
        let a = String::from("acgturymkwsbdhvn-");
        assert_eq!(complement(&a), "TGCAAYRKMWSVHDBN-");
        dbg!(a);
    }
}
