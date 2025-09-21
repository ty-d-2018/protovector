
pub mod mesh{
    use super::super::vector::{ 
        Vector, 
        BoxedVector, 
        Numeric 
    };
    use super::super::arrow::{
        Point, 
        Direction,
    };
    use super::sort::{ Inflect };
    use super::{ Operation };

    pub struct OpStage{
        pub lx: Numeric,
        pub ly: Numeric,
        pub lz: Numeric,
        pub ry: Numeric,
        pub rx: Numeric,
        pub rz: Numeric,
    }

    impl OpStage{
        pub fn new(x1: Numeric, y1: Numeric, z1: Numeric, x2: Numeric, y2: Numeric, z2: Numeric) -> OpStage{
            OpStage{
                lx: x1,
                ly: y1,
                lz: z1,
                rx: x2,
                ry: y2,
                rz: z2,
            }
        }
        pub fn get_nested(&self) -> ((Numeric, Numeric, Numeric), (Numeric, Numeric, Numeric)){
            ((self.lx, self.ly, self.lz), (self.rx, self.ry, self.rz))
        }
    }

    pub trait SimpleMesh: Vector{
        fn product(&self, stage: &OpStage) -> (Numeric, Numeric, Numeric){
            let ((lx, ly, lz), (rx, ry, rz)) = stage.get_nested();

            (lx * rx, ly * ry, lz * rz)
        }
        fn add(&self, stage: &OpStage) -> (Numeric, Numeric, Numeric){
            let ((lx, ly, lz), (rx, ry, rz)) = stage.get_nested();

            (lx + rx, ly + ry, lz + rz)
        }
        fn divide(&self, stage: &OpStage) -> (Numeric, Numeric, Numeric){
            let ((lx, ly, lz), (rx, ry, rz)) = stage.get_nested();

            (lx / rx, ly / ry, lz / rz)
        }
        fn subtract(&self, stage: &OpStage) -> (Numeric, Numeric, Numeric){
            let ((lx, ly, lz), (rx, ry, rz)) = stage.get_nested();

            (lx - rx, ly - ry, lz - rz)
        }
        fn simple_mesh(&self, inflect: &Inflect, left: &BoxedVector, right: &BoxedVector, operation: &Operation) -> BoxedVector{
            let (x1, y1, z1) = inflect.get_scalar(left);
            let (x2, y2, z2) = inflect.get_scalar(right);
            let stage = OpStage::new(x1, y1, z1, x2, y2, z2);
            let (xx, yy, zz) = match operation{
                Operation::Multiply => self.product(&stage),
                Operation::Add => self.add(&stage),
                Operation::Subtract => self.subtract(&stage),
                Operation::Divide => self.divide(&stage),
            };
            let vector: BoxedVector = left.create(xx, yy, zz);

            vector
        }
    }
}

pub enum Operation{
    Multiply,
    Add,
    Subtract,
    Divide,
}

pub mod sort{
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
