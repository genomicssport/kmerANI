mod args;
mod estimatekmers;
mod fasta;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::estimatekmers::kmerestimate;
use clap::Parser;
use figlet_rs::FIGfont;

/*
 Authom GauravSablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-9-23
*/

fn main() {
    let fontgenerate = FIGfont::standard().unwrap();
    let repgenerate = fontgenerate.convert("kmerANI");
    println!("{}", repgenerate.unwrap());
    let argsparse = CommandParse::parse();
    match &argsparse.command {
        Commands::ANIEstimate { filepath, kmer } => {
            let command = kmerestimate(filepath, kmer).unwrap();
            println!("The command has been finished:{}", command);
        }
    }
}
