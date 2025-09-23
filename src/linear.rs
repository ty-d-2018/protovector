
pub mod basic{
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
        lx: Numeric,
        ly: Numeric,
        lz: Numeric,
        ry: Numeric,
        rx: Numeric,
        rz: Numeric,
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
        pub fn get_half(&self, is_right: bool) -> (Numeric, Numeric, Numeric){
            if is_right{
                (self.rx, self.ry, self.rz)
            }else{
                (self.lx, self.ly, self.lz)
            }
        }
        pub fn third_half(&self, pos: char, is_right: bool) -> Option::<(Numeric, (Numeric, Numeric, Numeric))>{
            match (pos, is_right){
                ('x', true) => Some((self.lx, (self.rx, self.ry, self.rz))),
                ('x', false) => Some((self.rx, (self.lx, self.ly, self.lz))),
                ('y', true) => Some((self.ly, (self.rx, self.ry, self.rz))),
                ('y', false) => Some((self.ry, (self.lx, self.ly, self.lz))),
                ('z', true) => Some((self.lz, (self.rx, self.ry, self.rz))),
                ('z', false) => Some((self.rz, (self.lx, self.ly, self.lz))),
                _ => None,
            }
        }
        pub fn two_thirds(&self, pos_left: char, pos_right: char) -> Option::<(Numeric, Numeric)>{
            let l_numerial = match pos_left{
                'x' => self.lx,
                'y' => self.ly,
                'z' => self.lz,
                _ => {return None;},
            };
            let r_numerial = match pos_right{
                'x' => self.rx,
                'y' => self.ry,
                'z' => self.rz,
                _ => {return None;},
            };

            Some((l_numerial, r_numerial))
        }
    }

    pub trait MeshMix: Vector{
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
    pub trait ScalarMix: Vector{
        fn s_product(&self, stage: &OpStage, pos: char, right_side: bool) -> (Numeric, Numeric, Numeric){
            todo!();
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
    use super::super::error::{ VectorError };

    pub type VScalar = (Numeric, Numeric, Numeric);

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
        pub fn get_scalar(&self, vector: &BoxedVector) -> VScalar{
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
        pub fn scalar_product(&self, r_inflect: &Inflect, r_vector: &BoxedVector, l_vector: &BoxedVector) -> Result::<VScalar, VectorError>{
            let (scalar, _, _) = match self{
                Inflect::X | Inflect::Y | Inflect::Z => self.get_scalar(l_vector),
                _ => {return Err(VectorError::Coordinate);}
            };
            
            self.get_scalar(l_vector);
            let (x, y, z) = r_inflect.get_scalar(r_vector);

            Ok((scalar * x, scalar * y, scalar * z))
        }
        pub fn product(&self, r_inflect: &Inflect, r_vector: &BoxedVector, l_vector: &BoxedVector) -> VScalar{
            let (x, y, z) = self.get_scalar(l_vector);
            let (x2, y2, z2) = r_inflect.get_scalar(r_vector);

            (x * x2, y * y2, z * z2)
        }
        pub fn add(&self, r_inflect: &Inflect, r_vector: &BoxedVector, l_vector: &BoxedVector) -> VScalar{
            let (x, y, z) = self.get_scalar(l_vector);
            let (x2, y2, z2) = r_inflect.get_scalar(r_vector);

            (x + x2, y + y2, z + z2)
        }
        pub fn subtract(&self, r_inflect: &Inflect, r_vector: &BoxedVector, l_vector: &BoxedVector) -> VScalar{
            let (x, y, z) = self.get_scalar(l_vector);
            let (x2, y2, z2) = r_inflect.get_scalar(r_vector);

            (x - x2, y - y2, z - z2)
        }
        pub fn divide(&self, r_inflect: &Inflect, r_vector: &BoxedVector, l_vector: &BoxedVector) -> VScalar{
            let (x, y, z) = self.get_scalar(l_vector);
            let (x2, y2, z2) = r_inflect.get_scalar(r_vector);

            (x / x2, y / y2, z / z2)
        }
    }
}
