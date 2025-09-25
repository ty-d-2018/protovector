use super::vector::{ Vector };

pub type DynVector = dyn Vector;
pub type BoxedVector = Box::<DynVector>;
pub type Numeric = f32;
pub type VScalar = (Numeric, Numeric, Numeric);
pub type RNumerial = (u32, u32, u32);