use clap::Parser;

/// Simple program to generate long dog
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// How long should the dog be?
    #[arg(short, long, default_value_t = 1)]
    long: u8,
}

fn main() {
    let args = Args::parse();
    let long_dog = format!("L{}ong dog", "o".repeat(args.long as usize));
    println!("{}", long_dog);
}
