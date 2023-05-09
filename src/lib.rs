use element::{h, t, VElement};
use render::render;
use wasm_bindgen::prelude::*;
use web_sys::{window, Element, Node};

pub mod element;
pub mod render;

// Called by our JS entry point to run the example
#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let _app = h(
        "div",
        vec![("id", "app")],
        vec![
            h(
                "h1",
                vec![("style", "color: red;")],
                vec![t("Ciao mondo!!!"), h("br", vec![], vec![]), t("ooooo!!!")],
            ),
            h("br", vec![], vec![]),
            h(
                "img",
                vec![(
                    "src",
                    "https://media.giphy.com/media/cuPm4p4pClZVC/giphy.gif",
                )],
                vec![],
            ),
        ],
    );

    let app_node = render(&_app);

    match body.query_selector("#root") {
        Ok(_el) => {
            if _el.is_some() {
                let _el = _el.unwrap();
                _el.replace_with_with_node_1(&app_node).unwrap();
            }
        }
        Err(_) => (),
    }

    Ok(())
}
