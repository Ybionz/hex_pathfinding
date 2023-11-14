use wasm_bindgen::{prelude::Closure, JsValue};
use web_sys::MouseEvent;

#[derive(Clone)]
pub struct Event {
    pub event_type: String,
    pub js_value: JsValue,
}

impl Event {
    pub fn new(event_type: &str, input_closure: Closure<dyn FnMut(MouseEvent)>) -> Event {
        let string = String::from(event_type);
        let event = Event {
            event_type: string,
            js_value: input_closure.as_ref().clone(),
        };
        input_closure.forget();
        event
    }
}
