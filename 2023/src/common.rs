use std::{
    collections::HashMap,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use num::{Num, Signed};

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct Point2D<T>
where
    T: Num,
{
    pub x: T,
    pub y: T,
}

impl<T> Point2D<T>
where
    T: Num + Copy + Eq + Ord,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn in_range(&self, x_range: T, y_range: T) -> bool {
        return self.x >= T::zero() && self.x < x_range && self.y >= T::zero() && self.y < y_range;
    }

    pub fn move_dxy(&self, dx: T, dy: T) -> Self {
        return Self::new(self.x + dx, self.y + dy);
    }

    pub fn move_dir(&self, dir: Direction, steps: T) -> Point2D<T> {
        match dir {
            Direction::North => Point2D::new(self.x, self.y - steps),
            Direction::East => Point2D::new(self.x + steps, self.y),
            Direction::South => Point2D::new(self.x, self.y + steps),
            Direction::West => Point2D::new(self.x - steps, self.y),
        }
    }
}
impl<T> Point2D<T>
where
    T: Signed + Copy + Eq + Ord,
{
    pub fn manhattan_distance(&self, destination: &Point2D<T>) -> T {
        return (self.x - destination.x).abs() + (self.y - destination.y).abs();
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Point3D<T: Num> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point3D<T>
where
    T: Num,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Cube<T: Num> {
    pub low_corner: Point3D<T>,
    pub high_corner: Point3D<T>,
}

impl<T> Cube<T>
where
    T: Num
        + Eq
        + Ord
        + Copy
        + Add<Output = T>
        + Sub<Output = T>
        + AddAssign<T>
        + SubAssign<T>
        + From<i8>,
{
    pub fn new(low_corner: Point3D<T>, high_corner: Point3D<T>) -> Self {
        let min_x = *std::cmp::min(&low_corner.x, &high_corner.x);
        let min_y = *std::cmp::min(&low_corner.y, &high_corner.y);
        let min_z = *std::cmp::min(&low_corner.z, &high_corner.z);
        let max_x = *std::cmp::max(&low_corner.x, &high_corner.x);
        let max_y = *std::cmp::max(&low_corner.y, &high_corner.y);
        let max_z = *std::cmp::max(&low_corner.z, &high_corner.z);
        Self {
            low_corner: Point3D::new(min_x, min_y, min_z),
            high_corner: Point3D::new(max_x, max_y, max_z),
        }
    }

    pub fn contains(&self, coord: &Point3D<T>) -> bool {
        coord.x >= self.low_corner.x
            && coord.x <= self.high_corner.x
            && coord.y >= self.low_corner.y
            && coord.y <= self.high_corner.y
            && coord.z >= self.low_corner.z
            && coord.z <= self.high_corner.z
    }

    pub fn crosses(&self, other: &Cube<T>) -> bool {
        self.low_corner.x <= other.high_corner.x
            && self.high_corner.x >= other.low_corner.x
            && self.low_corner.y <= other.high_corner.y
            && self.high_corner.y >= other.low_corner.y
            && self.low_corner.z <= other.high_corner.z
            && self.high_corner.z >= other.low_corner.z
    }

    pub fn move_down(&mut self) {
        self.low_corner.z -= T::from(1);
        self.high_corner.z -= T::from(1);
    }

    pub fn move_up(&mut self) {
        self.low_corner.z += T::from(1);
        self.high_corner.z += T::from(1);
    }
}

#[repr(u8)]
#[derive(Eq, Debug, Copy, Clone, PartialEq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn from_char(c: char) -> Self {
        match c {
            'N' | 'U' => Direction::North,
            'E' | 'R' => Direction::East,
            'S' | 'D' => Direction::South,
            'W' | 'L' => Direction::West,
            _ => panic!("Invalid direction char"),
        }
    }
}

pub fn transpose<T: Clone>(array2d: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut result = Vec::<Vec<T>>::new();
    for x in 0..array2d[0].len() {
        let mut row = Vec::<T>::new();
        for y in 0..array2d.len() {
            row.push(array2d[y][x].clone());
        }
        result.push(row);
    }
    result
}

pub struct MapHashMap<K, V>
where
    K: Num,
{
    pub map: HashMap<Point2D<K>, V>,
}

impl<V> MapHashMap<i32, V> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

impl MapHashMap<i32, char> {
    pub fn parse_map(lines: &Vec<String>) -> Self {
        let mut map = Self::new();
        lines.iter().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                map.map.insert(Point2D::new(x as i32, y as i32), c);
            });
        });
        map
    }

    pub fn find(&self, c: char) -> Vec<&Point2D<i32>> {
        self.map
            .iter()
            .filter(|(_, &v)| v == c)
            .map(|(k, _)| k)
            .collect::<Vec<&Point2D<i32>>>()
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum EdgeType {
    Horizontal,
    Vertical,
    ULC,
    URC,
    DLC,
    DRC,
}

impl EdgeType {
    pub fn is_corner(&self) -> bool {
        match self {
            EdgeType::ULC => true,
            EdgeType::URC => true,
            EdgeType::DLC => true,
            EdgeType::DRC => true,
            _ => false,
        }
    }

    pub fn is_horizontal(&self) -> bool {
        match self {
            EdgeType::Horizontal => true,
            _ => false,
        }
    }

    pub fn is_vertical(&self) -> bool {
        match self {
            EdgeType::Vertical => true,
            _ => false,
        }
    }
}
