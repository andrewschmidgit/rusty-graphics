use clap::Parser;
use rasterizer::{parsing::Config, triangle::Triangle, image::ImageWrapper};

fn main() {
    let config = Config::parse();
    config.validate().unwrap();

    let triangle = Triangle::new(config.vertex_1, config.vertex_2, config.vertex_3);
    let mut image = ImageWrapper::new(config.width, config.height);

    let write_result = image.write(config.filename, triangle);
    match write_result {
        Ok(_) => println!("Successfully generated image"),
        Err(e) => eprintln!("{:?}", e),
    }
}
