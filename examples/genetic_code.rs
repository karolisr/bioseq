fn main() {
    // for i in 0..34 {
    //     let gcn = bioseq::GENETIC_CODE_NAMES[i];
    //     let gcaa = bioseq::GENETIC_CODE_AMINO_ACIDS[i];
    //     let gcss = bioseq::GENETIC_CODE_START_STOP[i];
    //     match gcn {
    //         "NA" => continue,
    //         _ => println!("{i:02}: {gcn}\n{gcaa}\n{gcss}\n"),
    //     }
    // }

    const GC_ID: usize = 1;

    let tt = bioseq::get_trans_table(GC_ID);
    dbg!(tt);

    let start_codons = bioseq::get_start_codons(GC_ID);
    dbg!(start_codons);

    let stop_codons = bioseq::get_stop_codons(GC_ID);
    dbg!(stop_codons);
}
