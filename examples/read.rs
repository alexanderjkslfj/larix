use larix::Item;

pub fn main() {
    const XML: &str = r#"<first><child></child><child></child></first>second<!-- third --><fourth happy="Yes" />"#;

    let items = Item::from_str(XML).unwrap();

    for item in &items {
        match item {
            Item::Element(element) => {
                if element.self_closing {
                    println!(
                        "Found an empty element. Is it happy? {}!",
                        element.attributes.get("happy").unwrap()
                    );
                } else {
                    println!(
                        "Found an element with {} children! Its tag name is \"{}\".",
                        element.children.len(),
                        element.name
                    );
                }
            }
            Item::Text(text) => {
                println!("Found some text. It says \"{}\"!", text);
            }
            Item::Comment(_) => {
                println!("Found a comment. Its raw XML looks like this: {}", item);
            }
            _ => println!("Found an unexpected item."),
        };
    }
}
