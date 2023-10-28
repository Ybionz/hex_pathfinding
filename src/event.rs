use js_sys::Function;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::MouseEvent;

pub struct Event {
    pub event_type: String,
    pub closure: Closure<dyn FnMut(MouseEvent)>,
}

impl Event {
    pub fn new(event_type: &str, input_closure: Closure<dyn FnMut(MouseEvent)>) -> Event {
        let string = String::from(event_type);
        // let js_closure = input_closure.as_ref().clone().unchecked_into();
        // let f = Function::new_no_args(js_closure);
        Event {
            event_type: string,
            // closure: js_closure,
            closure: input_closure,
        }
    }
    // pub fn new_with_js(event_type: &str, js_closure: js_sys::Function) -> Event {
    //     let string = String::from(event_type);
    //     Event {
    //         event_type: string,
    //         closure: js_closure,
    //     }
    // }
}

// use js_sys::Function;
// use wasm_bindgen::{prelude::Closure, JsCast};
// use web_sys::MouseEvent;

// pub struct Event<'a> {
//     pub event_type: String,
//     pub closure: &'a Closure<dyn FnMut(MouseEvent)>,
// }

// impl Event<'_> {
//     pub fn new<'a>(
//         event_type: &str,
//         input_closure: &'a Closure<dyn FnMut(MouseEvent)>,
//     ) -> Event<'a> {
//         let string = String::from(event_type);
//         // let js_closure = input_closure.as_ref().clone().unchecked_into();
//         // let f = Function::new_no_args(js_closure);
//         Event {
//             event_type: string,
//             // closure: js_closure,
//             closure: input_closure,
//         }
//     }
//     // pub fn new_with_js(event_type: &str, js_closure: js_sys::Function) -> Event {
//     //     let string = String::from(event_type);
//     //     Event {
//     //         event_type: string,
//     //         closure: js_closure,
//     //     }
//     // }
// }
