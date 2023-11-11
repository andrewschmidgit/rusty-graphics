use clap::Parser;
use rasterizer::{parsing::Config, triangle::Triangle, image::ImageWrapper, color::Color};

fn main() {
    let config = Config::parse();
    config.validate().unwrap();
    
    let red   = Color::new(255,  0 ,  0 );
    let green = Color::new( 0 , 255,  0 );
    let blue  = Color::new( 0 ,  0 , 255);

    let triangle = Triangle::new(config.v1, config.v2, config.v3, red, green, blue);
    let mut image = ImageWrapper::new(config.width, config.height);

    let write_result = image.write(config.filename, triangle);
    match write_result {
        Ok(_) => println!("Successfully generated image"),
        Err(e) => eprintln!("{:?}", e),
    }
}
