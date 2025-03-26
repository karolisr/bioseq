mod complementation;
mod genetic_code;
mod translation;

pub use complementation::{complement, rev_comp, reverse};
pub use genetic_code::{
    get_gen_code_name, get_start_codons, get_stop_codons, get_trans_table, validate_gc_id,
};
pub use translation::translate;
