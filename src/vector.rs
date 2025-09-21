pub type DynVector = dyn Vector;
pub type BoxedVector = Box::<DynVector>;
pub type Numeric = f32;

pub trait Vector{
    fn create(&self, x: Numeric, y: Numeric, z: Numeric) -> BoxedVector;
    fn get_dimension(&self) -> (Numeric, Numeric, Numeric);
    fn get_clone(&self) -> BoxedVector;
    fn get_x(&self) -> Numeric{
        let (x, y, z) = self.get_dimension();
        x
    }
    fn get_y(&self) -> Numeric{
        let (x, y, z) = self.get_dimension();
        y
    }
    fn get_z(&self) -> Numeric{
        let(x, y, z) = self.get_dimension();        
        z
    }
    fn get_xy(&self) -> (Numeric, Numeric){
        let(x, y, z) = self.get_dimension();
        (x, y)
    }
    fn get_xz(&self) -> (Numeric, Numeric){
        let (x, y, z) = self.get_dimension();
        (x, z)
    }
    fn get_yx(&self) -> (Numeric, Numeric){
        let (x, y, z) = self.get_dimension();
        (y, x)
    }
    fn get_yz(&self) -> (Numeric, Numeric){
        let (x, y, z) = self.get_dimension();
        (y, z)
    }
    fn get_zx(&self) -> (Numeric, Numeric){
        let (x, y, z) = self.get_dimension();
        (z, x)
    }
    fn get_zy(&self) -> (Numeric, Numeric){
        let (x, y, z) = self.get_dimension();
        (z, y)
    }
    fn get_matrix(&self) -> Vec<Numeric>{
        todo!();
    }
}