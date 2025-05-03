use clap::{Parser, ValueEnum};
use lambda_calculus_abstract_machine_kn::{
    eval,
    format::{blc, named},
};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
/// A small demo project performing normal-order evaluation of untyped lambda calculus terms.
/// It implements a full-reducing variant (KN) of the Krivine abstract machine;
/// specifically the formulation in Á. GARCÍA-PÉREZ and P. NOGUEIRA, "The full-reducing Krivine abstract machine KN simulates pure normal-order reduction in lockstep: A proof via corresponding calculus", Journal of Functional Programming, vol. 29, p. e7, 2019. doi:10.1017/S0956796819000017; Fig. 3. "A version of KN that works with closures with closed terms".
/// (This repo is unaffiliated; goal is me learning).
struct Cli {
    /// Lambda term that should be evaluated.
    term: String,

    #[arg(value_enum,  default_value_t=Format::Named)]
    /// Input format of the provided lambda term.
    format: Format,
}

#[derive(Clone, Debug, ValueEnum)]
enum Format {
    /// Typical representation using named variables and `λ`/`\` for lambdas. (Requires strict paranthesis).
    /// Example: ((λa. (a a)) (λa. a))
    Named,

    /// John Tromp's Binary Lambda Calculus encoding format
    /// (see https://tromp.github.io/cl/Binary_lambda_calculus.html#lambda_encoding).
    /// Example: 01000110100010
    Blc,
}

fn main() {
    let cli = Cli::parse();

    let term = match cli.format {
        Format::Named => named::decode(&cli.term),
        Format::Blc => blc::decode(&cli.term),
    };
    println!("parsed term:");
    println!("    named: {}", term);
    println!("    blc:   {}", blc::encode(&term));

    let reduced = eval(term);
    println!("reduced term:");
    println!("    named: {}", reduced);
    println!("    blc:   {}", blc::encode(&reduced));
}
