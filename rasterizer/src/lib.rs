pub mod parsing;

pub mod triangle {
    use std::ops::RangeInclusive;

    #[derive(Clone, Debug)]
    pub struct Point {
        pub x: u32, 
        pub y: u32,
    }

    impl Point {
        pub fn is(&self, (x, y): (u32, u32)) -> bool {
            self.x == x && self.y == y
        }
    }

    pub struct Triangle {
        p1: Point,
        p2: Point,
        p3: Point,
    }

    impl Triangle {
        pub fn new(p1: Point, p2: Point, p3: Point) -> Triangle {
            Triangle { p1, p2, p3 }
        }

        pub fn get_bounding_box(&self) -> BoundingBox {
            let xs = vec![self.p1.x, self.p2.x, self.p3.x];
            let ys = vec![self.p1.y, self.p2.y, self.p3.y];

            BoundingBox { 
                x0: *xs.iter().min().unwrap(),
                x1: *xs.iter().max().unwrap(),
                y0: *ys.iter().min().unwrap(),
                y1: *ys.iter().max().unwrap(),
            }
        }

        pub fn is_vertex(&self, point: (u32, u32)) -> bool {
            self.p1.is(point) || self.p2.is(point) || self.p3.is(point)
        }
    }

    #[derive(Debug)]
    pub struct BoundingBox {
        x0: u32,
        x1: u32,
        y0: u32,
        y1: u32,
    }

    impl BoundingBox {
        pub fn x(&self) -> RangeInclusive<u32> {
            self.x0..=self.x1
        }
        pub fn y(&self) -> RangeInclusive<u32> {
            self.y0..=self.y1
        }
    }
}

pub mod image {
    use image::{RgbImage, ImageBuffer, Rgb, ImageResult};

    use crate::triangle::Triangle;

    pub struct ImageWrapper {
        image: RgbImage,
    }

    impl ImageWrapper {
        pub fn new(width: u32, height: u32) -> ImageWrapper {
            let image = ImageBuffer::new(width, height);

            ImageWrapper { image }
        }

        pub fn write(&mut self, filename: String, triangle: Triangle) -> ImageResult<()> {
            let bounding_box = triangle.get_bounding_box();
            for x in bounding_box.x() {
                for y in bounding_box.y() {
                    let color: image::Rgb<u8> = 
                    if triangle.is_vertex((x, y)) { Rgb([255, 255, 255]) } 
                    else if x % 2 == 0 { Rgb([255, 0, 0]) }
                    else if x % 2 == 1 { Rgb([0, 255, 0]) }
                    else { Rgb([0, 0, 0]) };

                    self.image.put_pixel(x, y, color);
                }
            }

            self.image.save(filename)
        }
    }


}
