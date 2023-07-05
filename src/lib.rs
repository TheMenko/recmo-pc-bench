use clap::ValueEnum;

pub mod ark;
pub mod blst;
pub mod halo2;
pub mod plonky2;
pub mod pse;
pub mod winter;
pub mod risc0;

//pub use eth_stark;
pub use barretenberg;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
pub enum Algorithm {
    Blst,
    Halo2,
    PSE,
    Plonky2_Keccak,
    Plonky2_Poseidon,
    Ark,
    Winter,
    Risc0,
    //Stark,
    Barretenberg,
}

pub fn run(algorithm: Algorithm, max_exponent: usize) {
    use Algorithm::*;
    match algorithm {
        Blst => blst::run(max_exponent),
        Halo2 => halo2::run(max_exponent),
        PSE => pse::run(max_exponent),
        Plonky2_Keccak => plonky2::run(max_exponent, false),
        Plonky2_Poseidon => plonky2::run(max_exponent, true),
        Ark => ark::run(max_exponent),
        Winter => winter::run(max_exponent),
        Risc0 => risc0::run(max_exponent),
        //Stark => eth_stark::run(max_exponent, 4),
        Barretenberg => barretenberg::run(max_exponent),
    }
}
