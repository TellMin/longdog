mod long_dog_creator;

use crate::long_dog_creator::Dog;
use clap::Parser;
use image::ImageError;

/// Simple program to generate long dog
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// How long should the dog be?
    #[arg(short, long, default_value_t = 1)]
    long: u8,
}

fn main() -> Result<(), ImageError> {
    let args = Args::parse();
    Dog::new(args.long)?.save_long_dog("long_dog.png")?;
    Ok(())
}
