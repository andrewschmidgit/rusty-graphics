pub mod parsing;
pub mod color {
    use image::Rgb;

    #[derive(Debug, PartialEq, Eq, Clone, Default)]
    pub struct Color {
        pub r: u8,
        pub g: u8,
        pub b: u8,
    }

    impl Color {
        pub fn new(r: u8, g: u8, b: u8) -> Color {
            Color { r, g, b }
        }
    }
    
    impl Into<Rgb<u8>> for Color
    {
        fn into(self) -> Rgb<u8> {
            Rgb([self.r, self.b, self.g])
        }
    }
}

pub mod triangle {
    use std::ops::RangeInclusive;

    use crate::color::Color;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Point {
        pub x: u32,
        pub y: u32,
    }

    impl Point {
        fn sub(&self, rhs: &Self) -> Vector {
            Vector {
                u: self.x as f32 - rhs.x as f32,
                v: self.y as f32 - rhs.y as f32,
            }
        }
    }

    impl Point {
        pub fn new(x: u32, y: u32) -> Point {
            Point { x, y }
        }
    }

    pub struct Triangle {
        p1: Point,
        p2: Point,
        p3: Point,

        c1: Color,
        c2: Color,
        c3: Color,
    }

    impl Triangle {
        pub fn new(p1: Point, p2: Point, p3: Point, c1: Color, c2: Color, c3: Color) -> Triangle {
            Triangle { p1, p2, p3, c1, c2, c3 }
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

        pub fn is_vertex(&self, point: Point) -> bool {
            self.p1 == point || self.p2 == point || self.p3 == point
        }

        fn compute_barry(&self, point: &Point) -> (f32, f32, f32) {
            let v0 = self.p2.sub(&self.p1);
            let v1 = self.p3.sub(&self.p1);
            let v2 = point.sub(&self.p1);
            
            let d00 = v0.dot(&v0);
            let d01 = v0.dot(&v1);
            let d11 = v1.dot(&v1);
            let d20 = v2.dot(&v0);
            let d21 = v2.dot(&v1);
            let inv_denominator = 1.0 / (d00 * d11 - d01 * d01) as f32;
            
            let v = (d11 * d20 - d01 * d21) as f32 * inv_denominator;
            let w = (d00 * d21 - d01 * d20) as f32 * inv_denominator;
            let u = 1.0 - v - w;
            (u, v, w)
        }

        pub fn get_color(&self, point: &Point) -> Color {
            let (u, v, w) = self.compute_barry(point);

            let u_in_bounds = 0.0 <= u && u <= 1.0;
            let v_in_bounds = 0.0 <= v && v <= 1.0;
            let w_in_bounds = 0.0 <= w && w <= 1.0;

            if !u_in_bounds || !v_in_bounds || !w_in_bounds {
                return Color { r: 0, g: 0, b: 0 };
            }

            let r = self.c1.r as f32 * u + self.c2.r as f32 * v + self.c3.r as f32 * w;
            let r = r as u8;
            let g = self.c1.g as f32 * u + self.c2.g as f32 * v + self.c3.g as f32 * w;
            let g = g as u8;
            let b = self.c1.b as f32 * u + self.c2.b as f32 * v + self.c3.b as f32 * w;
            let b = b as u8;

            Color::new(r, g, b)
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

    #[derive(Debug)]
    pub struct Vector {
        u: f32,
        v: f32,
    }

    impl Vector {
        fn dot(&self, rhs: &Self) -> f32 {
            self.u * rhs.u + self.v * rhs.v
        }
    }
}

pub mod image {
    use image::{RgbImage, ImageBuffer, ImageResult};

    use crate::triangle::{Triangle, Point};

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
                    let point = Point { x, y };

                    let color: image::Rgb<u8> = triangle.get_color(&point).into();

                    self.image.put_pixel(x, y, color);
                }
            }

            self.image.save(filename)
        }
    }
}
