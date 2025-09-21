use super::vector::{
    Vector, 
    BoxedVector,
    Numeric,
};
use super::linear::Sort::{ Inflect };

#[derive(Clone)]
pub struct Direction{
    Inflect: Inflect,
    scalar: Point,
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
    pub fn new(Inflect: &Inflect, point: &Point) -> Direction{
        Direction{
            Inflect: Inflect.clone(),
            scalar: point.clone(),
        }
    }
}