use crate::element::VElement;
use web_sys::Node;

/// Handles the creating of a real element using the web_sys library
/// 1. creates an element consuming the tag name
/// 2. adds attributes to the element
/// 3. sets the text content as necessary
/// 4. returns the node straight to the caller
pub fn render(v_element: &VElement) -> Node {
    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("should have a document on window");

    let r_element = document
        .create_element(&v_element.tag_name.as_str())
        .unwrap();

    for (key, value) in &v_element.attributes {
        r_element.set_attribute(key, value).unwrap();
    }

    match &v_element.content {
        None => r_element.set_text_content(Some("")),
        Some(text) => r_element.set_text_content(Some(text.as_str())),
    }

    for child in v_element.children.iter() {
        if &child.tag_name == "__text_node" && child.content.is_some() {
            let text_content = child.content.clone();
            let text_node = document
                .create_text_node(text_content.unwrap().as_str())
                .get_root_node();
            r_element.append_child(&text_node).unwrap();
        } else {
            r_element.append_child(&render(child)).unwrap();
        }
    }

    r_element.get_root_node()
}
