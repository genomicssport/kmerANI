use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "kmerANI",
    version = "1.0",
    about = "kmerANI.
       ************************************************
      Gaurav Sablok, IBCH, PAN, Poznan, Poland,
      https://portal.ichb.pl/laboratory-of-genomics/.
      Email: gsablok@ibch.poznan.pl
      ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// kmer shared based identity
    ANIEstimate {
        /// path to the sequence file
        filepath: String,
        /// path to the output file
        kmer: String,
    },
}
