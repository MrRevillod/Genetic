
use lazy_static::lazy_static;
use std::{collections::HashMap, ops::Add};

/// Position type
/// 
/// Represents a point in the grid, it can be None
/// because the Entity can be initialized without 
/// a position on crossover

pub type Position = Option<Point>;

/// Point struct
/// 
/// Represents a point (x, y)

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub struct Point {
    pub x: isize,
    pub y: isize
}

impl Point {

    pub fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }
}

impl Add for Point {

    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

lazy_static!(

    /// ### DIRECTIONS constant array

    /// This constant array represents
    /// the 8 possible directions of movement

    pub static ref DIRECTIONS: [Point; 8] = [
        Point::new(-1, -1), // up-left
        Point::new(0, -1),  // up
        Point::new(1, -1),  // up-right
        Point::new(1, 0),   // right
        Point::new(1, 1),   // down-right
        Point::new(0, 1),   // down
        Point::new(-1, 1),  // down-left
        Point::new(-1, 0)   // left
    ];

    /// ### DEBUG_DIRECTIONS constant HashMap
    /// 
    /// This constant HashMap is used for debugging
    /// purposes, it maps the direction Point to a
    /// string representation

    pub static ref DEBUG_DIRECTIONS: HashMap<Point, &'static str> = HashMap::from([
        (DIRECTIONS[0], "up-left"),
        (DIRECTIONS[1], "up"),
        (DIRECTIONS[2], "up-right"),
        (DIRECTIONS[3], "right"),
        (DIRECTIONS[4], "down-right"),
        (DIRECTIONS[5], "down"),
        (DIRECTIONS[6], "down-left"),
        (DIRECTIONS[7], "left")
    ]);
);