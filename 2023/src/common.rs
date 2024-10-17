use num::Signed;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct Point2D<T> 
where T: Signed {
    pub x: T,
    pub y: T,
}

impl<T> Point2D<T> 
where T: Signed + Copy + Eq + Ord{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn manhattan_distance(&self, destination: &Point2D<T>) -> T 
    {
        return (self.x - destination.x).abs() + (self.y - destination.y).abs();
    }

    pub fn in_range(&self, x_range: T, y_range: T) -> bool {
        return self.x >= T::zero() && self.x < x_range && self.y >= T::zero() && self.y < y_range;
    }

    pub fn move_dxy(&self, dx: T, dy: T) -> Self {
        return Self::new(self.x + dx, self.y + dy);
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
