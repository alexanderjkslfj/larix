use std::{collections::HashMap, fmt::Display};

use crate::{stringify, Item};

/** Element ```<tag attr="value">...</tag>``` or ```<tag attr="value" />```. */
pub struct Element {
    /** Tag name of the element. */
    pub name: String,
    /** Items between the start and end tags of the element. */
    pub children: Vec<Item>,
    /** Attributes of the element. */
    pub attributes: HashMap<String, String>,
    /** Whether to self-close if childless. */
    pub self_closing: bool,
}

impl Element {
    pub fn new(name: String) -> Self {
        Element {
            name,
            children: Vec::new(),
            attributes: HashMap::new(),
            self_closing: false,
        }
    }

    /** Get all descendants matching the predicate.
    ```rust
    // Example of finding all elements with tag name "a":
    let xml = "<element><a></a><b><a></a></b><c>text</c></element>";

    # use larix::*;
    let Item::Element(element) = &parse(&xml)?[0] else {
        panic!();
    };

    let a_elements = element.find_descendants(&|item| {
        let Item::Element(el) = item else {
            return false;
        };
        el.name == "a"
    });

    assert_eq!(a_elements.len(), 2);
    # Ok::<(), Error>(())
    ```*/
    pub fn find_descendants(&self, predicate: &impl Fn(&Item) -> bool) -> Vec<&Item> {
        let mut result: Vec<&Item> = self
            .children
            .iter()
            .filter(|item| predicate(item))
            .collect();

        for child in &self.children {
            let Item::Element(element) = child else {
                continue;
            };
            result.append(&mut element.find_descendants(predicate));
        }

        result
    }

    /** Get the text content of all text items within the element.
    ```xml
    <element>Hello<child>World</child></element>
    ```
    The above would result in "HelloWorld".*/
    pub fn get_text_content(self: &Self) -> String {
        let mut content = String::new();

        for child in &self.children {
            match child {
                Item::Text(text) => {
                    content.push_str(&text);
                }
                Item::Element(element) => {
                    content.push_str(&element.get_text_content());
                }
                _ => (),
            }
        }

        content
    }

    /** Get all children which are elements. */
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

    /** Get all items at a certain depth within the element.
    ```xml
    <element>
        <item depth="1">
            <item at-depth="2">
                This text is at depth 3.
            </item>
        </item>
    </element>
    ```*/
    pub fn get_decendants_at_depth(self: &Self, depth: u8) -> Vec<&Item> {
        if depth == 0 {
            panic!("Depth cannot be zero.");
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
        let str = if self.self_closing {
            let mut str = get_start_tag(self);
            str.insert_str(str.len() - 1, " /");
            str
        } else {
            format!(
                "{}{}{}",
                get_start_tag(self),
                stringify(&self.children),
                get_end_tag(self)
            )
        };

        write!(f, "{str}")
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
