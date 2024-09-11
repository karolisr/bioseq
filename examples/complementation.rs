fn main() {
    let input = "ACGTURYMKWSBDHVN-XPZ-acgturymkwsbdhvn-xpz-";
    let comp1: String = bioseq::complementation::complement(input);
    println!("{input}");
    println!("{comp1}");
}
