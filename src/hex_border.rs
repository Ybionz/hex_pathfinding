use crate::f_point::FPoint;

#[derive(Clone)]
pub struct HexBorder {
    pub from: FPoint,
    pub to: FPoint,
    pub wall: bool,
}

impl HexBorder {
    pub fn new(from: FPoint, to: FPoint, wall: bool) -> HexBorder {
        HexBorder { from, to, wall }
    }
}
