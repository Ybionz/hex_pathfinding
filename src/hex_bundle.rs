use crate::hex::Hex;
use crate::hex_border::HexBorder;

#[derive(Clone)]
pub struct HexBundle {
    pub hex: Hex,
    pub edges: Vec<HexBorder>,
}

impl HexBundle {
    pub fn new(hex: Hex, edges: Vec<HexBorder>) -> HexBundle {
        HexBundle { hex, edges }
    }
}
