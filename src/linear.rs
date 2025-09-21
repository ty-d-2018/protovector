
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