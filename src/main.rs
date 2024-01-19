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

struct Dog {
    head: ImageBuffer<Rgba<u8>, Vec<u8>>,
    body: ImageBuffer<Rgba<u8>, Vec<u8>>,
    tail: ImageBuffer<Rgba<u8>, Vec<u8>>,
    long: u8,
}

impl Dog {
    fn new<P: AsRef<Path>>(head: P, body: P, tail: P, long: u8) -> Dog {
        Dog {
            head: load_image(head).unwrap(),
            body: load_image(body).unwrap(),
            tail: load_image(tail).unwrap(),
            long,
        }
    }

    fn width(&self) -> u32 {
        self.head.width() + self.body.width() * (self.long) as u32 + self.tail.width()
    }

    fn height(&self) -> u32 {
        self.head.height()
    }

    fn compose(&self, long: u8) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let (width, height) = (self.width(), self.height());
        let mut img = ImageBuffer::new(width, height);

        let mut x = 0;
        img.copy_from(&self.head, x, 0).unwrap();
        x += self.head.width();

        for _ in 0..long {
            img.copy_from(&self.body, x, 0).unwrap();
            x += self.body.width();
        }

        img.copy_from(&self.tail, x, 0).unwrap();

        img
    }
}

fn main() -> Result<(), ImageError> {
    let args = Args::parse();

    let dog = Dog::new(
        "images/data01.png",
        "images/data02.png",
        "images/data03.png",
        args.long,
    );

    let img = dog.compose(args.long);

    save_image(
        &img,
        format!("l{}ng_dog.png", "o".repeat(args.long as usize)).as_str(),
    )?;

    println!("D{}ne!", "o".repeat(args.long as usize));
    Ok(())
}

fn save_image(img: &ImageBuffer<Rgba<u8>, Vec<u8>>, filename: &str) -> Result<(), ImageError> {
    img.save(filename)
}

fn load_image<P: AsRef<Path>>(path: P) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, ImageError> {
    image::open(path).map(|img| img.to_rgba8())
}
