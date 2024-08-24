#[cfg(test)]
mod tests {
    use larix::Item;

    #[test]
    fn test_text() {
        const RAW: &str = "abcxyz";

        let items = Item::from_str(RAW).unwrap();
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

        let items = Item::from_str(RAW).unwrap();
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

        let items = Item::from_str(RAW).unwrap();
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

        let items = Item::from_str(RAW).unwrap();
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

        let items = Item::from_str(RAW).unwrap();
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

        let items = Item::from_str(RAW).unwrap();
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

        let items = Item::from_str(RAW).unwrap();
        assert_eq!(items.len(), 1);
        let inner = match &items[0] {
            Item::Element(e) => e,
            _ => panic!("Item is of wrong type."),
        };
        assert_eq!(inner.name, "a");
        assert_eq!(inner.attributes.len(), 0);
        assert_eq!(inner.children.len(), 0);
        assert_eq!(RAW, items[0].to_string());
        assert_eq!(RAW, inner.to_string());
    }

    #[test]
    fn test_empty_element() {
        const RAW: &str = "<a />";

        let items = Item::from_str(RAW).unwrap();
        assert_eq!(items.len(), 1);
        let inner = match &items[0] {
            Item::EmptyElement(e) => e,
            _ => panic!("Item is of wrong type."),
        };
        assert_eq!(inner.name, "a");
        assert_eq!(inner.attributes.len(), 0);
        assert_eq!(items[0].to_string(), RAW);
        assert_eq!(inner.to_string(), RAW);
    }

    #[test]
    fn test_element_with_attrs() {
        const RAW: &str = r#"<xyz tree="oak" material="wood"></xyz>"#;

        let items = Item::from_str(RAW).unwrap();
        assert_eq!(items.len(), 1);
        let inner = match &items[0] {
            Item::Element(e) => e,
            _ => panic!("Item is of wrong type."),
        };
        assert_eq!(inner.name, "xyz");
        assert_eq!(inner.children.len(), 0);
        assert_eq!(inner.attributes.len(), 2);
        assert!(inner.attributes.get("tree").is_some());
        assert!(inner.attributes.get("material").is_some());
        assert_eq!(inner.attributes.get("tree").unwrap(), "oak");
        assert_eq!(inner.attributes.get("material").unwrap(), "wood");
        assert_eq!(RAW, items[0].to_string());
        assert_eq!(RAW, inner.to_string());
    }

    #[test]
    fn test_empty_element_with_attrs() {
        const RAW: &str = r#"<xyz tree="oak" material="wood" />"#;

        let items = Item::from_str(RAW).unwrap();
        assert_eq!(items.len(), 1);
        let item = match &items[0] {
            Item::EmptyElement(e) => e,
            _ => panic!("Item is of wrong type."),
        };
        assert_eq!(item.name, "xyz");
        assert_eq!(item.attributes.len(), 2);
        assert!(item.attributes.get("tree").is_some());
        assert!(item.attributes.get("material").is_some());
        assert_eq!(item.attributes.get("tree").unwrap(), "oak");
        assert_eq!(item.attributes.get("material").unwrap(), "wood");
    }

    #[test]
    fn test_nesting() {
        const RAW: &str = "<a><b><c></c></b><c></c></a>";

        let items = Item::from_str(RAW).unwrap();
        assert_eq!(items.len(), 1);
        let inner = match &items[0] {
            Item::Element(e) => e,
            _ => panic!("Item is of wrong type."),
        };
        assert_eq!(inner.name, "a");
        assert_eq!(inner.children.len(), 2);
    }
}
