use image::{GenericImage, ImageBuffer, ImageError, Rgba};
use std::path::Path;

pub struct Dog {
    head: ImageBuffer<Rgba<u8>, Vec<u8>>,
    body: ImageBuffer<Rgba<u8>, Vec<u8>>,
    tail: ImageBuffer<Rgba<u8>, Vec<u8>>,
    long: u8,
}

impl Dog {
    pub fn new(long: u8) -> Result<Dog, ImageError> {
        Ok(Dog {
            head: load_image("images/data01.png")?,
            body: load_image("images/data02.png")?,
            tail: load_image("images/data03.png")?,
            long,
        })
    }

    fn width(&self) -> u32 {
        self.head.width() + self.body.width() * (self.long) as u32 + self.tail.width()
    }

    fn height(&self) -> u32 {
        self.head.height()
    }

    fn create_long_dog(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let (width, height) = (self.width(), self.height());
        let mut img = ImageBuffer::new(width, height);

        let mut x = 0;
        img.copy_from(&self.head, x, 0).unwrap();
        x += self.head.width();

        for _ in 0..self.long {
            img.copy_from(&self.body, x, 0).unwrap();
            x += self.body.width();
        }

        img.copy_from(&self.tail, x, 0).unwrap();

        img
    }

    pub fn save_long_dog(&self, filename: &str) -> Result<(), ImageError> {
        let img = self.create_long_dog();
        save_image(&img, filename)
    }
}

fn load_image<P: AsRef<Path>>(path: P) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, ImageError> {
    image::open(path).map(|img| img.to_rgba8())
}

fn save_image(img: &ImageBuffer<Rgba<u8>, Vec<u8>>, filename: &str) -> Result<(), ImageError> {
    img.save(filename)
}
