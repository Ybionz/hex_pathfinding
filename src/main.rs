use leptos::*;
use petgraph::prelude::GraphMap;
use petgraph::Undirected;

extern crate console_error_panic_hook;
use std::panic;

use crate::canvas::{add_event_listeners_to_canvas, context, draw_hex_grid};
use crate::graph::hex_graph_with_random_remove;
use crate::hex::Hex;

pub mod canvas;
pub mod constants;
pub mod f_point;
pub mod graph;
pub mod hex;
pub mod hex_bundle;
pub mod hex_border;

// console::log_1(&format!("Has wall").into());

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (graph, set_graph) = create_signal(hex_graph_with_random_remove(20, 20, 150));

    create_effect(move |_| {
        draw_hex_grid(&graph.get());
        add_event_listeners_to_canvas(&graph.get(), set_graph);
    });

    view! {
        <button
            on:click=move |_| {

            }
        >
            "Click me: "
            {1}
        </button>
        <MyCanvas graph=graph() set_graph=set_graph/>
    }
}

#[component]
fn MyCanvas(
    graph: GraphMap<Hex, i32, Undirected>,
    set_graph: WriteSignal<GraphMap<Hex, i32, Undirected>>,
) -> impl IntoView {
    // create_effect(move |_| {
    //     draw_hex_grid(&context(), &graph, set_graph);
    // });
}
