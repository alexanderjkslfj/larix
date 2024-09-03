use larix::{parse, stringify, Item};

pub fn main() {
    const XML: &str =
        r#"<first><bob></bob><alice></alice></first>second<!-- third --><fourth happy="Yes" />"#;

    // parse xml
    let mut items = parse(XML).unwrap();

    // remove last item
    items.pop();

    for item in &mut items {
        match item {
            Item::Element(element) => {
                // add the attribute "happy", and set it to "Very much so"
                element
                    .attributes
                    .insert(String::from("happy"), String::from("Very much so"));
                // add a child called "peter"
                element
                    .children
                    .push(Item::new_element(String::from("peter")));
            }
            Item::Text(text) => {
                // change the text from "second" to " Hello "
                text.replace_range(.., " Hello ");
            }
            Item::Comment(comment) => {
                // change the comment from " third " to " World "
                comment.replace_range(.., " World ");
            }
            _ => println!("Huh, that's odd."),
        };
    }

    println!("{}", stringify(&items));
}
