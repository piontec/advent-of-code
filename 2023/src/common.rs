use num::Signed;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Point2D<T> 
where T: Signed {
    pub x: T,
    pub y: T,
}

impl<T> Point2D<T> 
where T: Signed + Copy {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn manhattan_distance(&self, destination: &Point2D<T>) -> T 
    {
        return (self.x - destination.x).abs() + (self.y - destination.y).abs();
    }
}
