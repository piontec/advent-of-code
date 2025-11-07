use std::{
    collections::HashMap,
    fmt,
    ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign},
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

pub type Path<T> = Vec<Point2D<T>>;

impl<T> Point2D<T>
where
    T: Num + Copy + Eq + Ord,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn in_positive_range(&self, x_max_exclusive: T, y_max_exclusive: T) -> bool {
        return self.x >= T::zero()
            && self.x < x_max_exclusive
            && self.y >= T::zero()
            && self.y < y_max_exclusive;
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

    pub fn turn_cw(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn turn_ccw(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::West => Direction::East,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
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

impl<K: Num, V> MapHashMap<K, V> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

impl<K: Num, V> Index<Point2D<K>> for MapHashMap<K, V>
where
    K: Eq + std::hash::Hash,
{
    type Output = V;

    fn index(&self, index: Point2D<K>) -> &Self::Output {
        self.map.get(&index).unwrap()
    }
}

impl<K: Num, V> Index<&Point2D<K>> for MapHashMap<K, V>
where
    K: Eq + std::hash::Hash,
{
    type Output = V;

    fn index(&self, index: &Point2D<K>) -> &Self::Output {
        self.map.get(index).unwrap()
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

#[derive(Clone, Eq, PartialEq)]
pub struct MapVector<V> {
    pub map: Vec<Vec<V>>,
}

impl<V> MapVector<V> {
    pub fn new<F>(lines: &Vec<String>, convert: F) -> Self
    where
        F: Fn(char) -> V,
    {
        let map = lines
            .iter()
            .map(|line| line.chars().map(|c| convert(c)).collect::<Vec<V>>())
            .collect::<Vec<Vec<V>>>();
        Self { map }
    }

    pub fn is_in_map(&self, position: Point2D<isize>) -> bool {
        return position.x >= 0
            && position.y >= 0
            && position.y < self.map.len() as isize
            && position.x < self.map[0].len() as isize;
    }

    pub fn find(&self, what: V) -> Vec<Point2D<isize>>
    where
        V: PartialEq + Copy,
    {
        let mut res = vec![];
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                if self.map[y][x] == what {
                    res.push(Point2D::new(x as isize, y as isize));
                }
            }
        }
        res
    }

    pub fn find_in_front(
        &self,
        position: Point2D<isize>,
        direction: Direction,
        what: V,
    ) -> Option<Point2D<isize>>
    where
        V: PartialEq + Copy,
    {
        let mut current = position;
        loop {
            current = current.move_dir(direction, 1);
            if !current.in_positive_range(self.map[0].len() as isize, self.map.len() as isize) {
                return None;
            }
            if self[current] == what {
                return Some(current);
            }
        }
    }

    pub fn get_neighbors_pos<F>(&self, pos: &Point2D<isize>, mut filter: F) -> Vec<Point2D<isize>>
    where
        F: FnMut(&Point2D<isize>) -> bool,
    {
        let mut res = vec![];
        for direction in [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            let new_pos = pos.move_dir(direction, 1);
            if self.is_in_map(new_pos) && filter(&new_pos) {
                res.push(new_pos);
            }
        }
        res
    }

    pub fn get_neighbors<F>(&self, pos: &Point2D<isize>, filter: F) -> Vec<V>
    where
        F: FnMut(&Point2D<isize>) -> bool,
        V: Copy,
    {
        self.get_neighbors_pos(pos, filter)
            .iter()
            .map(|&p| self[p])
            .collect()
    }

    pub fn get_size(&self) -> Point2D<usize> {
        Point2D::new(self.map[0].len(), self.map.len())
    }
}

impl<V> fmt::Debug for MapVector<V>
where
    V: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                if let Err(err) = write!(f, "{}", self.map[y][x]) {
                    return Err(err);
                }
            }
            if let Err(err) = write!(f, "\n") {
                return Err(err);
            }
        }
        Ok(())
    }
}

impl<V> Index<Point2D<usize>> for MapVector<V> {
    type Output = V;

    fn index(&self, index: Point2D<usize>) -> &Self::Output {
        self.map.get(index.y).unwrap().get(index.x).unwrap()
    }
}

impl<V> Index<&Point2D<usize>> for MapVector<V> {
    type Output = V;

    fn index(&self, index: &Point2D<usize>) -> &Self::Output {
        self.map.get(index.y).unwrap().get(index.x).unwrap()
    }
}

impl<V> IndexMut<Point2D<usize>> for MapVector<V> {
    fn index_mut(&mut self, index: Point2D<usize>) -> &mut Self::Output {
        self.map.get_mut(index.y).unwrap().get_mut(index.x).unwrap()
    }
}

impl<V> IndexMut<&Point2D<usize>> for MapVector<V> {
    fn index_mut(&mut self, index: &Point2D<usize>) -> &mut Self::Output {
        self.map.get_mut(index.y).unwrap().get_mut(index.x).unwrap()
    }
}

impl<V> Index<Point2D<isize>> for MapVector<V> {
    type Output = V;

    fn index(&self, index: Point2D<isize>) -> &Self::Output {
        self.map
            .get(index.y as usize)
            .unwrap()
            .get(index.x as usize)
            .unwrap()
    }
}

impl<V> IndexMut<Point2D<isize>> for MapVector<V> {
    fn index_mut(&mut self, index: Point2D<isize>) -> &mut Self::Output {
        self.map
            .get_mut(index.y as usize)
            .unwrap()
            .get_mut(index.x as usize)
            .unwrap()
    }
}

impl<V> Index<&Point2D<isize>> for MapVector<V> {
    type Output = V;

    fn index(&self, index: &Point2D<isize>) -> &Self::Output {
        self.map
            .get(index.y as usize)
            .unwrap()
            .get(index.x as usize)
            .unwrap()
    }
}

impl<V> IndexMut<&Point2D<isize>> for MapVector<V> {
    fn index_mut(&mut self, index: &Point2D<isize>) -> &mut Self::Output {
        self.map
            .get_mut(index.y as usize)
            .unwrap()
            .get_mut(index.x as usize)
            .unwrap()
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
