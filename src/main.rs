use event::Event;
use leptos::*;
use petgraph::prelude::GraphMap;
use petgraph::Undirected;

extern crate console_error_panic_hook;
use std::panic;

use crate::canvas::{
    add_event_listeners_to_canvas, clear_canvas, draw_hex_graph, draw_hex_grid, remove_events,
};
use crate::enums::graph_style::GraphStyle;
use crate::graph::hex_graph_with_random_remove;
use crate::hex::Hex;

pub mod canvas;
pub mod constants;
mod enums;
mod event;
pub mod f_point;
pub mod graph;
pub mod hex;
pub mod hex_border;
pub mod hex_bundle;

// console::log_1(&format!("Has wall").into());

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (graph, set_graph) = create_signal(hex_graph_with_random_remove(20, 20, 550));

    let (style, set_style) = create_signal(GraphStyle::Grid);

    let (events, set_events ) = create_signal::<Vec<Event>>(Vec::new());

    create_effect(move |_| {
        events.with(|_events| remove_events(_events) );
        clear_canvas();
        if (style() == GraphStyle::Grid) {
            draw_hex_grid(&graph());
            // let e = add_event_listeners_to_canvas(&graph.get(), set_graph);
            // set_events(e);
        } else {
            draw_hex_graph(&graph())
        }
    });

    view! {
        <button
            on:click= move |_| set_style.set(style().reverse())
        >
            {move || format!("Change style to {:?}",style().reverse())}
        </button>
        // <MyCanvas graph=graph() set_graph=set_graph/>
    }
}

// #[component]
// fn MyCanvas(
//     graph: GraphMap<Hex, i32, Undirected>,
//     set_graph: WriteSignal<GraphMap<Hex, i32, Undirected>>,
// ) -> impl IntoView {
//     // create_effect(move |_| {
//     //     draw_hex_grid(&context(), &graph, set_graph);
//     // });
// }
