use std::{collections::HashMap, fmt::Display};

use crate::Element;

/** Empty element ```<tag attr="value" />```. */
pub struct EmptyElement {
    /** Tag name of the element. */
    pub name: String,
    /** Attributes of the element. */
    pub attributes: HashMap<String, String>,
}

impl EmptyElement {
    pub fn new(name: String) -> Self {
        EmptyElement {
            name,
            attributes: HashMap::new(),
        }
    }
}

impl Display for EmptyElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = get_empty_tag(self);
        write!(f, "{str}")
    }
}

impl TryFrom<Element> for EmptyElement {
    type Error = (); // TODO: use fitting error type

    fn try_from(value: Element) -> Result<Self, Self::Error> {
        if value.children.len() == 0 {
            Ok(EmptyElement {
                name: value.name,
                attributes: value.attributes,
            })
        } else {
            Err(())
        }
    }
}

fn get_empty_tag(element: &EmptyElement) -> String {
    let mut attributes = String::new();

    for attr in &element.attributes {
        attributes.push_str(&format!(r#" {}="{}""#, attr.0, attr.1));
    }

    format!("<{}{} />", element.name, attributes)
}
