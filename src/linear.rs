
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
        pub fn get_2d(one: char, two: char) -> Option::<Inflect>{
            match (one, two){
                ('x', 'y') => Some(Inflect::XY),
                ('x', 'z') => Some(Inflect::XZ),
                ('y', 'x') => Some(Inflect::YX),
                ('y', 'z') => Some(Inflect::YZ),
                ('z', 'x') => Some(Inflect::ZX),
                ('z', 'y') => Some(Inflect::ZY),
                _ => None,
            }
        }
    }
}
