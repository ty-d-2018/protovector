use super::vector::{
    Vector, 
    BoxedVector,
    Numeric,
};

#[derive(Clone)]
pub enum Axis{
    X,
    Y,
    Z,
    XY,
    XZ,
    YX,
    YZ,
    ZX,
    ZY,
    XYZ,
    XZY,
    YXZ,
    ZXY,
    YZX,
    ZYX,
}

#[derive(Clone)]
pub struct Direction{
    axis: Axis,
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
    pub fn new(axis: &Axis, point: &Point) -> Direction{
        Direction{
            axis: axis.clone(),
            scalar: point.clone(),
        }
    }
}

impl Axis{
    pub fn get_scalar(&self, vector: &BoxedVector) -> (Numeric, Numeric, Numeric){
        match self{
            Axis::X => {
                let x = vector.get_x();
                (x, 0.0, 0.0)
            },
            Axis::Y => {
                let y = vector.get_y();
                (y, 0.0, 0.0)
            },
            Axis::Z => {
                let z = vector.get_z();
                (z, 0.0, 0.0)
            },
            Axis::XY => {
                let (x, y) = vector.get_xy();
                (x, y, 0.0)
            },
            Axis::XZ => {
                let (x, z) = vector.get_xz();
                (x, z, 0.0)
            },
            Axis::YX => {
                let (y, x) = vector.get_yx();
                (y, x, 0.0)
            },
            Axis::YZ => {
                let (y, z) = vector.get_yz();
                (y, z, 0.0)
            }
            Axis::ZX => {
                let (z, x) = vector.get_zx();
                (z, x, 0.0)
            },
            Axis::ZY => {
                let (z, y) = vector.get_yz();
                (z, y, 0.0)
            },
            Axis::XYZ => {
                let (x, y, z) = vector.get_dimension();
                (x, y, z)
            },
            Axis::XZY => {
                let (x, y, z) = vector.get_dimension();
                (x, z, y)
            }
            Axis::YXZ => {
                let (x, y, z) = vector.get_dimension();
                (y, x, z)
            },
            Axis::ZXY => {
                let (x, y, z) = vector.get_dimension();
                (z, x, y)
            },
            Axis::YZX => {
                let (x, y, z) = vector.get_dimension();
                (y, z, x)
            }
            Axis::ZYX => {
                let (x, y, z) = vector.get_dimension();
                (z, y, x)
            }
        }
    }
}