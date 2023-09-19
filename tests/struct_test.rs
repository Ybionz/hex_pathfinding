use hex_pathfinding::f_point::*;
use hex_pathfinding::hex::*;

#[test]
fn f_point_new() {
    let f_point = FPoint::new(32.0, 33.0);
    assert_eq!(f_point.x, 32.0);
    assert_eq!(f_point.y, 33.0);
}

#[test]
fn hex_new() {
    let hex = Hex::new(2, 3);
    assert_eq!(hex.col, 2);
    assert_eq!(hex.row, 3);
}

#[test]
fn hex_build_inside_range() {
    let hex = Hex::build(2, 3);
    assert!(hex.is_some());
    assert_eq!(hex.unwrap().col, 2);
    assert_eq!(hex.unwrap().row, 3);
}

#[test]
fn hex_build_outside_range() {
    let hex = Hex::build(-1, 3);
    assert!(hex.is_none());
    let hex = Hex::build(1, -3);
    assert!(hex.is_none());
}

#[test]
fn hex_corners() {
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
