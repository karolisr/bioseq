fn main() {
    let input = "ACGTURYMKWSBDHVN-XPZ-acgturymkwsbdhvn-xpz-";
    let comp1: String = bioseq::complement(input);
    println!("{input}");
    println!("{comp1}");
}
