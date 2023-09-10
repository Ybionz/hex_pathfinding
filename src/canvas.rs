use crate::consts::*;
use crate::f_point::FPoint;
use crate::hex::Hex;
use leptos::document;
use petgraph::prelude::GraphMap;
use petgraph::Undirected;
use std::iter::zip;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;
use web_sys::Path2d;

pub fn context() -> CanvasRenderingContext2d {
    document()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap()
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}

pub fn draw_hex_grid(context: &CanvasRenderingContext2d, graph: GraphMap<Hex, i32, Undirected>) {
    graph.nodes().into_iter().for_each(|n| {
        let neighbors = graph.neighbors(n);
        draw_hex(context, n, neighbors);
    });
}

fn draw_hex(
    context: &CanvasRenderingContext2d,
    node: Hex,
    neighbors: petgraph::graphmap::Neighbors<'_, Hex>,
) -> () {
    if neighbors.clone().count() == 0 {
        draw_black_hex(node, context);
        return;
    }

    let mut from_points = node.corners();
    from_points.remove(from_points.len() - 1);
    let mut to_points = node.corners();
    to_points.remove(0);

    for ((from, to), neighbor) in zip(zip(from_points, to_points), node.neighbors()) {
        let wall = match neighbor {
            None => true,
            Some(ne) => !(neighbors.clone().any(|n| n.eq(&ne))),
        };
        draw_line_between(from, to, context, wall);
    }
}

fn draw_black_hex(node: Hex, context: &CanvasRenderingContext2d) {
    let path = Path2d::new().unwrap();
    for (i, p) in node.corners().into_iter().enumerate() {
        if i == 0 {
            path.move_to(p.x, p.y);
        } else {
            path.line_to(p.x, p.y);
        }
    }
    context.set_fill_style(&JsValue::from_str("black"));
    context.fill_with_path_2d(&path);
}

fn draw_line_between(from: FPoint, to: FPoint, context: &CanvasRenderingContext2d, wall: bool) {
    let path = Path2d::new().unwrap();
    path.move_to(from.x, from.y);
    path.line_to(to.x, to.y);
    if wall {
        context.set_line_width(WALL_THICKNESS);
    } else {
        context.set_line_width(1.);
    }
    context.stroke_with_path(&path);
}
