use clap::Parser;
use image::{GenericImage, ImageBuffer, ImageError};

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

    let img1 = image::open("images/data01.png").unwrap().to_rgba8();
    let img2 = image::open("images/data02.png").unwrap().to_rgba8();
    let img3 = image::open("images/data03.png").unwrap().to_rgba8();

    let (width, height) = (
        img1.width() + img2.width() * args.long as u32 + img3.width(),
        img1.height(),
    );

    let mut img = ImageBuffer::new(width, height);

    img.copy_from(&img1, 0, 0).unwrap();

    for i in 0..args.long {
        img.copy_from(&img2, img1.width() + img2.width() * i as u32, 0)
            .unwrap();
    }

    img.copy_from(&img3, img1.width() + img2.width() * args.long as u32, 0)
        .unwrap();

    img.save(format!("l{}ng_dog.png", "o".repeat(args.long as usize)))
        .unwrap();

    println!("D{}ne!", "o".repeat(args.long as usize));
    Ok(())
}
