use crate::f_point::FPoint;
use crate::constants::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hex {
    pub col: i32,
    pub row: i32,
}
impl Hex {
    pub fn new(col: i32, row: i32) -> Hex {
        Hex { col, row }
    }

    pub fn build(col: i32, row: i32) -> Option<Hex> {
        if col < 0 || row < 0 {
            return None;
        }
        return Some(Hex::new(col, row));
    }
    pub fn neighbors(self) -> Vec<Option<Hex>> {
        let mut vec_options = Vec::new();
        let col = self.col;
        let row = self.row;

        if (row % 2) == 0 {
            vec_options.push(Hex::build(col, row - 1));
            vec_options.push(Hex::build(col + 1, row));
            vec_options.push(Hex::build(col, row + 1));
            vec_options.push(Hex::build(col - 1, row + 1));
            vec_options.push(Hex::build(col - 1, row));
            vec_options.push(Hex::build(col - 1, row - 1));
        } else {
            vec_options.push(Hex::build(col + 1, row - 1));
            vec_options.push(Hex::build(col + 1, row));
            vec_options.push(Hex::build(col + 1, row + 1));
            vec_options.push(Hex::build(col, row + 1));
            vec_options.push(Hex::build(col - 1, row));
            vec_options.push(Hex::build(col, row - 1));
        }
        vec_options
    }

    pub fn corners(self) -> Vec<FPoint> {
        let x = START_X + f64::from(self.col) * *W * 2. + *W * f64::from(self.row % 2);
        let y = START_Y + f64::from(self.row) * H * 3. / 2.;
        vec![
            FPoint::new(x, y - H),
            FPoint::new(x + *W, y - H / 2.),
            FPoint::new(x + *W, y + H / 2.),
            FPoint::new(x, y + H),
            FPoint::new(x - *W, y + H / 2.),
            FPoint::new(x - *W, y - H / 2.),
            FPoint::new(x, y - H),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let hex = Hex::new(2, 3);
        assert_eq!(hex.col, 2);
        assert_eq!(hex.row, 3);
    }

    #[test]
    fn build_inside_range() {
        let hex = Hex::build(2, 3);
        assert!(hex.is_some());
        assert_eq!(hex.unwrap().col, 2);
        assert_eq!(hex.unwrap().row, 3);
    }

    #[test]
    fn build_outside_range() {
        let hex = Hex::build(-1, 3);
        assert!(hex.is_none());
        let hex = Hex::build(1, -3);
        assert!(hex.is_none());
    }

    #[test]
    fn corners() {
        let hex = Hex::new(0, 0);
        let corners = hex.corners();
        assert_eq!(*corners.get(0).unwrap(), FPoint::new(20.0, 0.0));
        assert_eq!(
            *corners.get(1).unwrap(),
            FPoint::new(37.32050807568877, 10.0)
        );
        assert_eq!(
            *corners.get(2).unwrap(),
            FPoint::new(37.32050807568877, 30.0)
        );
        assert_eq!(*corners.get(3).unwrap(), FPoint::new(20.0, 40.0));
        assert_eq!(
            *corners.get(4).unwrap(),
            FPoint::new(2.679491924311229, 30.0)
        );
        assert_eq!(
            *corners.get(5).unwrap(),
            FPoint::new(2.679491924311229, 10.0)
        );
        assert_eq!(*corners.get(6).unwrap(), FPoint::new(20.0, 0.));
    }
}
