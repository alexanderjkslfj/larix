use std::fmt::Display;

use crate::Element;

/** Any XML item. May be a comment, an element, a bit of text, ... */
pub enum Item {
    /** Element ```<tag attr="value">...</tag>``` or ```<tag attr="value" />```. */
    Element(Element),
    /** Comment ```<!-- ... -->```. */
    Comment(String),
    /** Escaped character data between tags. */
    Text(String),
    /** Document type definition data (DTD) stored in ```<!DOCTYPE ...>```. */
    DocType(String),
    /** Unescaped character data stored in ```<![CDATA[...]]>```. */
    CData(String),
    /** XML declaration ```<?xml ...?>```. */
    Decl(String),
    /** Processing instruction ```<?...?>```. */
    PI(String),
}

impl Item {
    pub fn new_element(name: String) -> Item {
        Item::Element(Element::new(name))
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = &match self {
            Self::Element(element) => element.to_string(),
            Self::Text(text) => text.to_owned(),
            Self::Comment(comment) => format!("<!--{comment}-->"),
            Self::DocType(doctype) => format!("<!DOCTYPE {doctype}>"),
            Self::Decl(decl) => format!("<?{decl}?>"),
            Self::CData(cdata) => format!("<![CDATA[{cdata}]]>"),
            Self::PI(pi) => format!("<?{pi}?>"),
        };

        write!(f, "{str}")
    }
}