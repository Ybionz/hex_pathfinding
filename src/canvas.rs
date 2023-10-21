use crate::constants::*;
use crate::f_point::FPoint;
use crate::graph::remove_all_edges_for_node;
use crate::hex::Hex;
use crate::hex_border::HexBorder;
use crate::hex_bundle::HexBundle;
use leptos::document;
use leptos::SignalUpdate;
use leptos::WriteSignal;
use petgraph::prelude::GraphMap;
use petgraph::Undirected;
use std::cell::Cell;
use std::iter::zip;
use std::sync::Arc;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::AddEventListenerOptions;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;
use web_sys::MouseEvent;
use web_sys::Path2d;

pub fn context() -> CanvasRenderingContext2d {
    canvas()
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}

fn build_hex_bundles(graph: &GraphMap<Hex, i32, Undirected>) -> Vec<HexBundle> {
    let mut hex_bundles: Vec<HexBundle> = Vec::with_capacity(graph.nodes().len());
    graph.nodes().into_iter().for_each(|n| {
        hex_bundles.push(HexBundle::new(
            n,
            hex_edges_for_node(n, &graph.neighbors(n)),
        ));
    });
    hex_bundles
}

pub fn draw_hex_grid(graph: &GraphMap<Hex, i32, Undirected>) {
    graph.nodes().into_iter().for_each(|n| {
        draw_hex(&context(), n, &graph.neighbors(n));
    });
}

fn hover_hex_closure(hex_bundles: Vec<HexBundle>) -> Closure<dyn FnMut(MouseEvent)> {
    let red_hex: Arc<Cell<Option<HexBundle>>> = Arc::new(Cell::new(None));
    let hover_closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
        if let Some(bundle) = red_hex.take() {
            draw_bundle(&bundle, false, &context())
        }
        (&hex_bundles)
            .into_iter()
            .filter(|bundle| {
                context().is_point_in_path_with_path_2d_and_f64(
                    &f_points_to_path(bundle.hex.corners()),
                    event.offset_x() as f64,
                    event.offset_y() as f64,
                )
            })
            .for_each(|bundle| {
                draw_bundle(&bundle, true, &context());
                red_hex.set(Some(bundle.clone()));
            });
    });
    hover_closure
}

fn add_click_event(
    hex_bundles: Vec<HexBundle>,
    hover_closure: Closure<dyn FnMut(MouseEvent)>,
    set_graph: WriteSignal<GraphMap<Hex, i32, Undirected>>,
) {
    let remove_closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
        (&hex_bundles)
            .into_iter()
            .filter(|bundle| {
                context().is_point_in_path_with_path_2d_and_f64(
                    &f_points_to_path(bundle.hex.corners()),
                    event.offset_x() as f64,
                    event.offset_y() as f64,
                )
            })
            .for_each(|bundle| {
                let _ = canvas().remove_event_listener_with_callback(
                    "mousemove",
                    hover_closure.as_ref().unchecked_ref(),
                );
                set_graph.update(|graph| {
                    remove_all_edges_for_node(graph, bundle.hex);
                });
            });
    });
    let _ = canvas().add_event_listener_with_callback_and_add_event_listener_options(
        "click",
        remove_closure.as_ref().unchecked_ref(),
        AddEventListenerOptions::new().once(true),
    );
    remove_closure.forget();
}

pub fn add_event_listeners_to_canvas(
    graph: &GraphMap<Hex, i32, Undirected>,
    set_graph: WriteSignal<GraphMap<Hex, i32, Undirected>>,
) {
    let hex_bundles: Vec<HexBundle> = build_hex_bundles(graph);
    let hover_closure = hover_hex_closure(hex_bundles.clone());
    let _ = canvas()
        .add_event_listener_with_callback("mousemove", hover_closure.as_ref().unchecked_ref());
    add_click_event(hex_bundles, hover_closure, set_graph);
}

fn canvas() -> HtmlCanvasElement {
    document()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap()
}

fn draw_bundle(hex_bundle: &HexBundle, hover: bool, context: &CanvasRenderingContext2d) {
    let colour = if hex_bundle.edges.clone().into_iter().all(|edge| edge.wall) {
        "black"
    } else if hover {
        "red"
    } else {
        "white"
    };

    draw_filled_hex(hex_bundle.hex, context, colour);

    draw_hex_edges(hex_bundle.edges.clone(), context);
}

fn draw_hex(
    context: &CanvasRenderingContext2d,
    node: Hex,
    connected_neighbors: &petgraph::graphmap::Neighbors<'_, Hex>,
) -> () {
    if connected_neighbors.clone().count() == 0 {
        draw_filled_hex(node, context, "black");
    }

    let mut from_points = node.corners();
    from_points.remove(from_points.len() - 1);
    let mut to_points = node.corners();
    to_points.remove(0);

    for ((from, to), neighbor) in zip(zip(from_points, to_points), node.neighbors()) {
        let wall = match neighbor {
            None => true,
            Some(ne) => !(connected_neighbors.clone().any(|n| n.eq(&ne))),
        };
        draw_line_between(from, to, wall, context);
    }
}

fn hex_edges_for_node(
    node: Hex,
    connected_neighbors: &petgraph::graphmap::Neighbors<'_, Hex>,
) -> Vec<HexBorder> {
    let mut vec = Vec::new();
    let mut from_points = node.corners();
    from_points.remove(from_points.len() - 1);
    let mut to_points = node.corners();
    to_points.remove(0);

    for ((from, to), neighbor) in zip(zip(from_points, to_points), node.neighbors()) {
        let wall = match neighbor {
            None => true,
            Some(ne) => !(connected_neighbors.clone().any(|n| n.eq(&ne))),
        };
        vec.push(HexBorder::new(from, to, wall));
    }
    vec
}

fn draw_hex_edges(edges: Vec<HexBorder>, context: &CanvasRenderingContext2d) {
    edges
        .into_iter()
        .for_each(|edge| draw_line_between(edge.from, edge.to, edge.wall, context));
}

fn draw_filled_hex(node: Hex, context: &CanvasRenderingContext2d, color: &str) {
    let path = f_points_to_path(node.corners());
    context.set_fill_style(&JsValue::from_str(color));
    context.fill_with_path_2d(&path);
}

fn f_points_to_path(fpoint: Vec<FPoint>) -> Path2d {
    let path = Path2d::new().unwrap();
    for (i, p) in fpoint.into_iter().enumerate() {
        if i == 0 {
            path.move_to(p.x, p.y);
        } else {
            path.line_to(p.x, p.y);
        }
    }
    path
}

fn draw_line_between(from: FPoint, to: FPoint, wall: bool, context: &CanvasRenderingContext2d) {
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
