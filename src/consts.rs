pub const SIZE: f64 = 20.;
pub const H: f64 = SIZE;
pub const START_X: f64 = 20.;
pub const START_Y: f64 = 20.;
pub const WALL_THICKNESS: f64 = 3.;

lazy_static! {
    pub static ref W: f64 = (3.0_f64).sqrt() / 2. * SIZE;
}
