#[cfg(test)]
mod tests {
    use larix::{parse, Item};

    #[test]
    fn test_text() {
        const RAW: &str = "abcxyz";

        let items = parse(RAW).unwrap();
        assert_eq!(items.len(), 1);
        let inner = match &items[0] {
            Item::Text(e) => e,
            _ => panic!("Item is of wrong type."),
        };
        assert_eq!(items[0].to_string(), RAW);
        assert_eq!(inner, RAW);
    }

    #[test]
    fn test_cdata() {
        const RAW: &str = "<![CDATA[abcxyz]]>";

        let items = parse(RAW).unwrap();
        assert_eq!(items.len(), 1);
        let inner = match &items[0] {
            Item::CData(e) => e,
            _ => panic!("Item is of wrong type."),
        };
        assert_eq!(items[0].to_string(), RAW);
        assert_eq!(inner, "abcxyz");
    }

    #[test]
    fn test_comment() {
        const RAW: &str = "<!-- abcxyz -->";

        let items = parse(RAW).unwrap();
        assert_eq!(items.len(), 1);
        let inner = match &items[0] {
            Item::Comment(e) => e,
            _ => panic!("Item is of wrong type."),
        };
        assert_eq!(items[0].to_string(), RAW);
        assert_eq!(inner, " abcxyz ");
    }

    #[test]
    fn test_doctype() {
        const RAW: &str = r#"<!DOCTYPE html
     PUBLIC "-//W3C//DTD XHTML 1.0 Strict//EN"
     "http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd">"#;

        let items = parse(RAW).unwrap();
        assert_eq!(items.len(), 1);
        let inner: &String = match &items[0] {
            Item::DocType(e) => e,
            _ => panic!("Item is of wrong type."),
        };
        assert_eq!(items[0].to_string(), RAW);
        assert_eq!(
            inner,
            r#"html
     PUBLIC "-//W3C//DTD XHTML 1.0 Strict//EN"
     "http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd""#
        );
    }

    #[test]
    fn test_decl() {
        const RAW: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#;

        let items = parse(RAW).unwrap();
        assert_eq!(items.len(), 1);
        let inner = match &items[0] {
            Item::Decl(e) => e,
            _ => panic!("Item is of wrong type."),
        };
        assert_eq!(items[0].to_string(), RAW);
        assert_eq!(
            inner,
            r#"xml version="1.0" encoding="UTF-8" standalone="no""#
        );
    }

    #[test]
    fn test_pi() {
        const RAW: &str = r#"<?notxml something="else" ?>"#;

        let items = parse(RAW).unwrap();
        assert_eq!(items.len(), 1);
        let inner = match &items[0] {
            Item::PI(e) => e,
            _ => panic!("Item is of wrong type."),
        };
        let text = items[0].to_string();
        assert_eq!(text, RAW);
        assert_eq!(inner, r#"notxml something="else" "#);
    }

    #[test]
    fn test_element() {
        const RAW: &str = "<a></a>";

        let items = parse(RAW).unwrap();
        assert_eq!(items.len(), 1);
        let element = match &items[0] {
            Item::Element(e) => e,
            _ => panic!("Item is of wrong type."),
        };
        assert_eq!(element.self_closing, false);
        assert_eq!(element.name, "a");
        assert_eq!(element.attributes.len(), 0);
        assert_eq!(element.children.len(), 0);
        assert_eq!(RAW, items[0].to_string());
        assert_eq!(RAW, element.to_string());
    }

    #[test]
    fn test_empty_element() {
        const RAW: &str = "<a />";

        let items = parse(RAW).unwrap();
        assert_eq!(items.len(), 1);
        let element = match &items[0] {
            Item::Element(e) => e,
            _ => panic!("Item is of wrong type."),
        };
        assert_eq!(element.self_closing, true);
        assert_eq!(element.name, "a");
        assert_eq!(element.attributes.len(), 0);
        assert_eq!(items[0].to_string(), RAW);
        assert_eq!(element.to_string(), RAW);
    }

    #[test]
    fn test_element_with_attrs() {
        const RAW: &str = r#"<xyz tree="oak" material="wood"></xyz>"#;
        const RAW_ALT: &str = r#"<xyz material="wood" tree="oak"></xyz>"#;

        let items = parse(RAW).unwrap();
        assert_eq!(items.len(), 1);
        let element = match &items[0] {
            Item::Element(e) => e,
            _ => panic!("Item is of wrong type."),
        };
        assert_eq!(element.self_closing, false);
        assert_eq!(element.name, "xyz");
        assert_eq!(element.children.len(), 0);
        assert_eq!(element.attributes.len(), 2);
        assert!(element.attributes.get("tree").is_some());
        assert!(element.attributes.get("material").is_some());
        assert_eq!(element.attributes.get("tree").unwrap(), "oak");
        assert_eq!(element.attributes.get("material").unwrap(), "wood");
        let item_str = items[0].to_string();
        assert!(RAW == item_str || RAW_ALT == item_str);
        let element_str = element.to_string();
        assert!(RAW == element_str || RAW_ALT == element_str);
    }

    #[test]
    fn test_empty_element_with_attrs() {
        const RAW: &str = r#"<xyz tree="oak" material="wood" />"#;
        const RAW_ALT: &str = r#"<xyz material="wood" tree="oak" />"#;

        let items = parse(RAW).unwrap();
        assert_eq!(items.len(), 1);
        let element = match &items[0] {
            Item::Element(e) => e,
            _ => panic!("Item is of wrong type."),
        };
        assert_eq!(element.self_closing, true);
        assert_eq!(element.name, "xyz");
        assert_eq!(element.attributes.len(), 2);
        assert!(element.attributes.get("tree").is_some());
        assert!(element.attributes.get("material").is_some());
        assert_eq!(element.attributes.get("tree").unwrap(), "oak");
        assert_eq!(element.attributes.get("material").unwrap(), "wood");
        let item_str = items[0].to_string();
        println!("{}", item_str);
        assert!(RAW == item_str || RAW_ALT == item_str);
        let element_str = element.to_string();
        assert!(RAW == element_str || RAW_ALT == element_str);
    }

    #[test]
    fn test_nesting() {
        const RAW: &str = "<a><b><c></c></b><c></c></a>";

        let items = parse(RAW).unwrap();
        assert_eq!(items.len(), 1);
        let inner = match &items[0] {
            Item::Element(e) => e,
            _ => panic!("Item is of wrong type."),
        };
        assert_eq!(inner.name, "a");
        assert_eq!(inner.children.len(), 2);
    }

    #[test]
    fn test_descendants_at_depth() {
        const RAW: &str = "<a><b><c></c></b><c></c></a>";

        let items = parse(RAW).unwrap();
        assert_eq!(items.len(), 1);
        let inner = match &items[0] {
            Item::Element(e) => e,
            _ => panic!("Item is of wrong type."),
        };
        assert_eq!(inner.get_decendants_at_depth(1).len(), 2);
        assert_eq!(inner.get_decendants_at_depth(2).len(), 1);
    }
}
