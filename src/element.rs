use std::{collections::HashMap, fmt::Display};

use crate::{EmptyElement, Item};

/** Element ```<tag attr="value">...</tag>```. */
pub struct Element {
    /** Tag name of the element. */
    pub name: String,
    /** Items between the start and end tags of the element. */
    pub children: Vec<Item>,
    /** Attributes of the element. */
    pub attributes: HashMap<String, String>,
}

impl Element {
    pub fn new(name: String) -> Self {
        Element {
            name,
            children: Vec::new(),
            attributes: HashMap::new(),
        }
    }

    pub fn get_child_elements(self: &Self) -> Vec<&Element> {
        let mut elements = Vec::new();

        for child in &self.children {
            let Item::Element(element) = child else {
                continue;
            };
            elements.push(element)
        }

        elements
    }

    pub fn get_decendants_at_depth(self: &Self, depth: u8) -> Vec<&Item> {
        if depth == 0 {
            panic!("Depth must be above zero.");
        }
        if depth == 1 {
            return self.children.iter().collect();
        }

        self.children
            .iter()
            .filter_map(|item| {
                let Item::Element(element) = item else {
                    return None;
                };
                Some(element.get_decendants_at_depth(depth - 1))
            })
            .reduce(|mut acc, mut curr| {
                acc.append(&mut curr);
                acc
            })
            .unwrap_or(Vec::new())
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = format!(
            "{}{}{}",
            get_start_tag(self),
            Item::to_str(&self.children),
            get_end_tag(self)
        );

        write!(f, "{str}")
    }
}

impl From<EmptyElement> for Element {
    fn from(value: EmptyElement) -> Self {
        Element {
            name: value.name,
            attributes: value.attributes,
            children: Vec::new(),
        }
    }
}

fn get_start_tag(element: &Element) -> String {
    let mut attributes = String::new();

    for attr in &element.attributes {
        attributes.push_str(&format!(r#" {}="{}""#, attr.0, attr.1));
    }

    format!("<{}{}>", element.name, attributes)
}

fn get_end_tag(element: &Element) -> String {
    format!("</{}>", element.name)
}
