use clap::Parser;
use image::{GenericImage, ImageBuffer, ImageError, Rgba};
use std::path::Path;

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

    let img1 = load_image("images/data01.png")?;
    let img2 = load_image("images/data02.png")?;
    let img3 = load_image("images/data03.png")?;

    let img = conpose_images(&img1, &img2, &img3, args.long);

    save_image(
        &img,
        format!("l{}ng_dog.png", "o".repeat(args.long as usize)).as_str(),
    )?;

    println!("D{}ne!", "o".repeat(args.long as usize));
    Ok(())
}

fn load_image<P: AsRef<Path>>(path: P) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, ImageError> {
    let image = image::open(path)?.to_rgba8();
    Ok(image)
}

fn conpose_images(
    img1: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    img2: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    img3: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    long: u8,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = (
        img1.width() + img2.width() * long as u32 + img3.width(),
        img1.height(),
    );

    let mut img = ImageBuffer::new(width, height);
    img.copy_from(img1, 0, 0).unwrap();

    for i in 0..long {
        img.copy_from(img2, img1.width() + img2.width() * i as u32, 0)
            .unwrap();
    }

    img.copy_from(img3, img1.width() + img2.width() * long as u32, 0)
        .unwrap();

    img
}

fn save_image(img: &ImageBuffer<Rgba<u8>, Vec<u8>>, filename: &str) -> Result<(), ImageError> {
    img.save(filename)
}
