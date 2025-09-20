pub type DynVector = dyn Vector;
pub type BoxedVector = Box::<DynVector>;

pub trait Vector{
    fn get_dimension(&self) -> (f32, f32, f32);
    fn get_clone(&self) -> BoxedVector;
    fn get_x(&self) -> f32{
        let (x, y, z) = self.get_dimension();
        x
    }
    fn get_y(&self) -> f32{
        let (x, y, z) = self.get_dimension();
        y
    }
    fn get_z(&self) -> f32{
        let(x, y, z) = self.get_dimension();        
        z
    }
    fn get_xy(&self) -> (f32, f32){
        let(x, y, z) = self.get_dimension();
        (x, y)
    }
    fn get_xz(&self) -> (f32, f32){
        let (x, y, z) = self.get_dimension();
        (x, z)
    }
    fn get_yx(&self) -> (f32, f32){
        let (x, y, z) = self.get_dimension();
        (y, x)
    }
    fn get_yz(&self) -> (f32, f32){
        let (x, y, z) = self.get_dimension();
        (y, z)
    }
    fn get_zx(&self) -> (f32, f32){
        let (x, y, z) = self.get_dimension();
        (z, x)
    }
    fn get_zy(&self) -> (f32, f32){
        let (x, y, z) = self.get_dimension();
        (z, y)
    }
    fn get_matrix(&self) -> Vec<f32>{
        todo!();
    }
}