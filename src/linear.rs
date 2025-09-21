
pub mod mesh{
    use super::super::vector::{ Vector, BoxedVector, Numeric };
    pub trait SimpleMesh: Vector{
        fn product(left: &BoxedVector, right: &BoxedVector) -> (Numeric, Numeric, Numeric){
            let (left_x, left_y, left_z) = left.get_dimension();
            let (right_x, right_y, right_z) = right.get_dimension();

            (left_x * right_x, left_y * right_y, left_z * right_z)
        }
        fn add(left: &BoxedVector, right: &BoxedVector) -> (Numeric, Numeric, Numeric){
            let (left_x, left_y, left_z) = left.get_dimension();
            let (right_x, right_y, right_z) = right.get_dimension();

            (left_x + right_x, left_y + right_y, left_z + right_z)
        }
        fn divide(left: &BoxedVector, right: &BoxedVector) -> (Numeric, Numeric, Numeric){
            let (left_x, left_y, left_z) = left.get_dimension();
            let (right_x, right_y, right_z) = right.get_dimension();

            (left_x / right_x, left_y / right_y, left_z / right_z)
        }
        fn subtract(left: &BoxedVector, right: &BoxedVector) -> (Numeric, Numeric, Numeric){
            let (left_x, left_y, left_z) = left.get_dimension();
            let (right_x, right_y, right_z) = right.get_dimension();

            (left_x - right_x, left_y - right_y, left_z - right_z)
        }
    }
}

pub enum Operation{
    Multiply,
    Add,
    Subtract,
    Divide,
}

pub mod Sort{
    use super::super::vector::{ Vector, BoxedVector, Numeric };

    #[derive(Clone)]
    pub enum Inflect{
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
    
    impl Inflect{
        pub fn get_scalar(&self, vector: &BoxedVector) -> (Numeric, Numeric, Numeric){
            match self{
                Inflect::X => {
                    let x = vector.get_x();
                    (x, 0.0, 0.0)
                },
                Inflect::Y => {
                    let y = vector.get_y();
                    (y, 0.0, 0.0)
                },
                Inflect::Z => {
                    let z = vector.get_z();
                    (z, 0.0, 0.0)
                },
                Inflect::XY => {
                    let (x, y) = vector.get_xy();
                    (x, y, 0.0)
                },
                Inflect::XZ => {
                    let (x, z) = vector.get_xz();
                    (x, z, 0.0)
                },
                Inflect::YX => {
                    let (y, x) = vector.get_yx();
                    (y, x, 0.0)
                },
                Inflect::YZ => {
                    let (y, z) = vector.get_yz();
                    (y, z, 0.0)
                }
                Inflect::ZX => {
                    let (z, x) = vector.get_zx();
                    (z, x, 0.0)
                },
                Inflect::ZY => {
                    let (z, y) = vector.get_yz();
                    (z, y, 0.0)
                },
                Inflect::XYZ => {
                    let (x, y, z) = vector.get_dimension();
                    (x, y, z)
                },
                Inflect::XZY => {
                    let (x, y, z) = vector.get_dimension();
                    (x, z, y)
                }
                Inflect::YXZ => {
                    let (x, y, z) = vector.get_dimension();
                    (y, x, z)
                },
                Inflect::ZXY => {
                    let (x, y, z) = vector.get_dimension();
                    (z, x, y)
                },
                Inflect::YZX => {
                    let (x, y, z) = vector.get_dimension();
                    (y, z, x)
                }
                Inflect::ZYX => {
                    let (x, y, z) = vector.get_dimension();
                    (z, y, x)
                }
            }
        }
    }
}
