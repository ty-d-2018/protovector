use super::vector::{
    Vector, 
    BoxedVector,
    Numeric,
};
use super::linear::sort::{ Inflect };

#[derive(Clone)]
pub struct Direction{
    axis: Inflect,
    point: Point,
}

#[derive(Clone)]
pub struct Point{
    x: Numeric,
    y: Numeric,
    z: Numeric,
}

impl Point{
    pub fn new(x: Numeric, y: Numeric, z: Numeric) -> Point{
        Point{
            x,
            y,
            z,
        }
    }
}

impl Vector for Point{
    fn create(&self, x: Numeric, y: Numeric, z: Numeric) -> BoxedVector{
        let point: Point = Point::new(x, y, z);
        Box::new(point)
    }
    fn get_dimension(&self) -> (Numeric, Numeric, Numeric){
        (self.x, self.y, self.z)
    }
    fn get_clone(&self) -> BoxedVector{
        Box::new(self.clone())
    }
}

impl Direction{
    pub fn new(axis: &Inflect, point: &Point) -> Direction{
        Direction{
            axis: axis.clone(),
            point: point.clone(),
        }
    }
}

impl Vector for Direction{
    fn create(&self, x: Numeric, y: Numeric, z: Numeric) -> BoxedVector{
        let point: Point = Point::new(x, y, z);
        let direction: Direction = Direction::new(&Inflect::XYZ, &point);

        Box::new(direction)
    }
    fn get_dimension(&self) -> (Numeric, Numeric, Numeric){
        let scalar: (Numeric, Numeric, Numeric) = self.axis.get_scalar(&self.point.get_clone());

        scalar
    }
    fn get_clone(&self) -> BoxedVector{
        Box::new(self.clone())
    }
}