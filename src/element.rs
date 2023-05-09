use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VElement {
    pub tag_name: String,
    pub attributes: Vec<(String, String)>,
    pub children: Vec<Self>,
    pub content: Option<String>,
}

/// Implements creating an element with a tag, or a text node
impl VElement {
    pub fn new(
        tag_name: String,
        attributes: Option<Vec<(String, String)>>,
        children: Option<Vec<Self>>,
    ) -> Self {
        Self {
            tag_name: tag_name,
            attributes: attributes.unwrap_or(vec![]),
            children: children.unwrap_or(vec![]),
            content: None,
        }
    }
    pub fn from_text(content: String) -> Self {
        Self {
            tag_name: String::from("__text_node"),
            attributes: vec![],
            children: vec![],
            content: Some(content),
        }
    }
}

/// Sugar coating the tag node creation
pub fn h(tag_name: &str, attributes: Vec<(&str, &str)>, children: Vec<VElement>) -> VElement {
    VElement::new(
        tag_name.to_string(),
        Some(
            attributes
                .iter()
                .map(|tuple| (tuple.0.to_string(), tuple.1.to_string()))
                .collect(),
        ),
        Some(children),
    )
}

/// Sugar coating creating a simple text node
pub fn t(content: &str) -> VElement {
    VElement::from_text(content.to_string())
}
