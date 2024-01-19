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

    let images = load_images(&[
        "images/data01.png",
        "images/data02.png",
        "images/data03.png",
    ])?;

    let img = conpose_images(&images, args.long);

    save_image(
        &img,
        format!("l{}ng_dog.png", "o".repeat(args.long as usize)).as_str(),
    )?;

    println!("D{}ne!", "o".repeat(args.long as usize));
    Ok(())
}

fn load_images<P: AsRef<Path>>(
    paths: &[P],
) -> Result<Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>, ImageError> {
    paths
        .iter()
        .map(|path| image::open(path).map(|img| img.to_rgba8()))
        .collect()
}

fn conpose_images(
    images: &[ImageBuffer<Rgba<u8>, Vec<u8>>],
    long: u8,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = (
        images.iter().map(|img| img.width()).sum(),
        images.iter().map(|img| img.height()).max().unwrap(),
    );

    let mut img = ImageBuffer::new(width, height);

    let mut x = 0;
    for i in 0..long {
        img.copy_from(&images[i as usize], x, 0).unwrap();
        x += images[i as usize].width();
    }

    img
}

fn save_image(img: &ImageBuffer<Rgba<u8>, Vec<u8>>, filename: &str) -> Result<(), ImageError> {
    img.save(filename)
}
