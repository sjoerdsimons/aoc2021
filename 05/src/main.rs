use anyhow::{Context, Result};
use std::cmp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> Result<()> {
    let file = File::open("05.txt")?;
    let mut grid_points = HashSet::new();
    let mut danger_points = HashSet::new();

    for input_line in BufReader::new(file).lines() {
        let line: Line = input_line?.parse()?;
        if let Some(points) = line.points() {
            for point in points {
                if grid_points.contains(&point) {
                    danger_points.insert(point);
                }
                grid_points.insert(point);
            }
        }
    }
    println!("{:#?}", danger_points);
    println!("{}", danger_points.len());

    Ok(())
}

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').context("Failed to split point")?;
        Ok(Point {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        let ord_x = self.x.cmp(&other.x);
        let ord_y = self.y.cmp(&other.y);
        match (ord_x, ord_y) {
            (ord_x, ord_y) if ord_x == ord_y => Some(ord_x),
            (ord_x, cmp::Ordering::Equal) => Some(ord_x),
            (cmp::Ordering::Equal, ord_y) => Some(ord_y),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn start(&self) -> Point {
        self.start
    }

    pub fn end(&self) -> Point {
        self.end
    }

    pub fn orientation(&self) -> Option<Orientation> {
        if self.start.x == self.end.x {
            Some(Orientation::Horizontal)
        } else if self.start.y == self.end.y {
            Some(Orientation::Vertical)
        } else {
            None
        }
    }

    pub fn points(&self) -> Option<DiscreteLine> {
        if self.orientation().is_some() {
            Some(DiscreteLine {
                remaining: Some(*self),
            })
        } else {
            None
        }
    }
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p0, p1) = s.split_once(" -> ").context("Failed to split line")?;
        let p0 = p0.parse()?;
        let p1 = p1.parse()?;

        if p0 < p1 {
            Ok(Line { start: p0, end: p1 })
        } else {
            Ok(Line { start: p1, end: p0 })
        }
    }
}

pub struct DiscreteLine {
    remaining: Option<Line>,
}

impl Iterator for DiscreteLine {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let remaining = self.remaining.as_mut()?;
        let item = remaining.start;
        if remaining.start == remaining.end {
            self.remaining = None;
        } else {
            match remaining.orientation().expect("No orientation") {
                Orientation::Horizontal => remaining.start.y += 1,
                Orientation::Vertical => remaining.start.x += 1,
            }
        }
        Some(item)
    }
}

#[test]
fn test_parse_line() {
    assert_eq!(
        "6,4 -> 2,0".parse::<Line>().unwrap(),
        Line {
            start: Point { x: 2, y: 0 },
            end: Point { x: 6, y: 4 },
        }
    );
}

#[test]
fn test_parse_point() {
    assert_eq!(
        "2,1 -> 2,1".parse::<Line>().unwrap(),
        Line {
            start: Point { x: 2, y: 1 },
            end: Point { x: 2, y: 1 },
        }
    );
}

#[test]
fn test_line_to_points() {
    let line = Line {
        start: Point { x: 1, y: 1 },
        end: Point { x: 1, y: 3 },
    };
    let expected = vec![
        Point { x: 1, y: 1 },
        Point { x: 1, y: 2 },
        Point { x: 1, y: 3 },
    ];
    assert_eq!(line.points().unwrap().collect::<Vec<_>>(), expected)
}

#[test]
fn test_point_to_points() {
    let line = Line {
        start: Point { x: 2, y: 1 },
        end: Point { x: 2, y: 1 },
    };
    let expected = vec![Point { x: 2, y: 1 }];
    assert_eq!(line.points().unwrap().collect::<Vec<_>>(), expected)
}
