use std::str::FromStr;
use clap::Parser;

use crate::triangle::Point;

pub struct ParseVertexError;

impl FromStr for Point {
    type Err = ParseVertexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .and_then(|s| s.split_once(','))
            .ok_or(ParseVertexError)?;

        let x_fromstr: u32 = x.trim().parse().map_err(|_| ParseVertexError)?;
        let y_fromstr: u32 = y.trim().parse().map_err(|_| ParseVertexError)?;

        Ok(Point { x: x_fromstr, y: y_fromstr })
    }
}

fn parse_vertex(s: &str) -> Result<Point, String> {
    s.parse().map_err(|_| "Could not parse into vertex. Should be in the form `(x, y)".into())
}

#[derive(Parser)]
pub struct Config {
    #[arg()]
    pub filename: String,

    #[arg(long, default_value_t = 512)]
    pub width: u32,

    #[arg(long, default_value_t = 512)]
    pub height: u32,

    #[arg(long, value_parser = parse_vertex)]
    pub vertex_1: Point,
    #[arg(long, value_parser = parse_vertex)]
    pub vertex_2: Point,
    #[arg(long, value_parser = parse_vertex)]
    pub vertex_3: Point,
}

impl Config {
    pub fn validate(&self) -> Result<(), String> {
        let vertex_validator = |v: &Point| -> Result<(), String> {
            match v {
                Point { x, y: _ } if x >= &self.width => {
                    Err(format!("Given x: {} is greater than the image height: {}", x, self.width - 1))
                },
                Point { x: _, y } if y >= &self.height => {
                    Err(format!("Given y: {} is greater than the image height: {}", y, self.height - 1))
                },
                _ => Ok(())
            }
        };

        vertex_validator(&self.vertex_1)?;
        vertex_validator(&self.vertex_2)?;
        vertex_validator(&self.vertex_3)?;

        Ok(())
    }
}
