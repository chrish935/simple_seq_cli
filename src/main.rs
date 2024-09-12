use clap::Parser;
use simple_seq_cli::{ transcribe_dna_to_rna, translate_rna_to_aa };

#[derive(Parser, Debug)]
#[clap(
    version = "1.0",
    author = "Christopher Horn <chrish935@gmail.com>",
    about = "A simple program to transcribe/translate DNA/RNA sequences"
)]
struct Opts {
    #[clap(short, long, help = "DNA sequence to transcribe")]
    dna: Option<String>,
    #[clap(short, long, help = "RNA sequence to translate")]
    rna: Option<String>,
}


fn main() {
    let opts = Opts::parse();
    
    if let Some(dna_seq) = opts.dna {
        let rna_seq = transcribe_dna_to_rna(&dna_seq);
        match rna_seq {
            Ok(rna) => println!("RNA sequence: {}", rna),
            Err(err) => {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }
        };
    } else if let Some(rna_seq) = opts.rna {
        let aa_seq = translate_rna_to_aa(&rna_seq);
        println!("AA sequence: {}", aa_seq);
    } else {
        eprintln!("Error: You must provide either a DNA or RNA sequence");
        std::process::exit(1);
    }
}
