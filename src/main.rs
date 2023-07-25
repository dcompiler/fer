use clap::Parser;

//use rgsl::types::*;
use rgsl::randist::binomial::binomial_pdf;

#[derive(Parser)]
//#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    long: f64,
    #[arg(short, long)]
    short: f64,
    #[arg(short, long)]
    plong: f64,
}

fn main() {

    let args = Args::parse();

    let pcs = args.long * args.plong + args.short * (1.0 - args.plong);

    let ibase = (args.long * args.plong + 1.0).floor() as u32;

    let ulong = args.long as u32;
    
    let mut overage = 0.0;
    for i in ibase .. (ulong +1) {
	overage += binomial_pdf(i, args.plong, ulong)
	    * (i as f64 - args.plong*args.long)*(args.long - args.short) / args.long;
    }

    let unstored = (args.plong * args.long.powi(2) + (1.0 - args.plong) * args.short.powi(2))
	/ 2.0 / pcs;
    
    // binomial_P(k: u32, p: f64, n: u32) -> f64
	
    println!("{}", overage/unstored);
}
