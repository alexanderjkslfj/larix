use std::{collections::HashMap, fmt::Display, string::FromUtf8Error};

use quick_xml::{
    errors::IllFormedError,
    events::{attributes::Attribute, BytesStart, Event},
    name::QName,
    Reader,
};

use crate::{Element, EmptyElement, Error};

/** Any XML item. May be a comment, an element, a bit of text, ... */
pub enum Item {
    /** Element ```<tag attr="value">...</tag>```. */
    Element(Element),
    /** Empty element ```<tag attr="value" />```. */
    EmptyElement(EmptyElement),
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
    pub fn new_empty_element(name: String) -> Item {
        Item::EmptyElement(EmptyElement::new(name))
    }

    /** Stringifies a list of XML items into valid XML. */
    pub fn to_str(xml: &Vec<Item>) -> String {
        let mut result = String::new();
        for item in xml {
            result.push_str(&item.to_string())
        }
        result
    }

    /** Parse XML. Text is trimmed. */
    pub fn from_str_trimmed(value: &str) -> Result<Vec<Self>, Error> {
        let events = get_all_events(value, true)?;
        Self::from_events(&(events.iter().collect::<Vec<&Event>>())[..])
    }

    /** Parse XML. */
    pub fn from_str(value: &str) -> Result<Vec<Self>, Error> {
        let events = get_all_events(value, false)?;
        Self::from_events(&(events.iter().collect::<Vec<&Event>>())[..])
    }

    fn non_decodable<T, U>(res: Result<T, FromUtf8Error>) -> Result<U, Error> {
        Err(Error::NonDecodable(Some(res.err().unwrap().utf8_error())))
    }

    fn from_events(events: &[&Event]) -> Result<Vec<Item>, Error> {
        let mut children: Vec<Item> = Vec::new();

        let mut i = 0;
        while i < events.len() {
            match &events[i] {
                Event::Text(e) => {
                    let str_res = u8_to_string(&e);
                    let Ok(str) = str_res else {
                        return Self::non_decodable(str_res);
                    };
                    children.push(Item::Text(str));
                }
                Event::Comment(e) => {
                    let str_res = u8_to_string(&e);
                    let Ok(str) = str_res else {
                        return Self::non_decodable(str_res);
                    };
                    children.push(Item::Comment(str));
                }
                Event::DocType(e) => {
                    let str_res = u8_to_string(&e);
                    let Ok(str) = str_res else {
                        return Self::non_decodable(str_res);
                    };
                    children.push(Item::DocType(str));
                }
                Event::CData(e) => {
                    let str_res = u8_to_string(&e);
                    let Ok(str) = str_res else {
                        return Self::non_decodable(str_res);
                    };
                    children.push(Item::CData(str));
                }
                Event::Decl(e) => {
                    let str_res = u8_to_string(&e);
                    let Ok(str) = str_res else {
                        return Self::non_decodable(str_res);
                    };
                    children.push(Item::Decl(str));
                }
                Event::PI(e) => {
                    let str_res = u8_to_string(&e);
                    let Ok(str) = str_res else {
                        return Self::non_decodable(str_res);
                    };
                    children.push(Item::PI(str));
                }
                Event::Empty(e) => {
                    let name_res = get_name(e);
                    let Ok(name) = name_res else {
                        return Self::non_decodable(name_res);
                    };
                    let attr_res = get_attributes(e);
                    let Ok(attributes) = attr_res else {
                        return Self::non_decodable(attr_res);
                    };

                    children.push(Item::EmptyElement(EmptyElement { name, attributes }))
                }
                Event::Start(e) => {
                    let name_res = get_name(e);
                    let Ok(name) = name_res else {
                        return Self::non_decodable(name_res);
                    };
                    let attr_res = get_attributes(e);
                    let Ok(attributes) = attr_res else {
                        return Self::non_decodable(attr_res);
                    };

                    let mut nested_events = Vec::new();
                    let mut names = vec![name.clone()];

                    loop {
                        i += 1;

                        match events[i] {
                            Event::Start(e) => {
                                let name_res = get_name(e);
                                let Ok(name) = name_res else {
                                    return Self::non_decodable(name_res);
                                };
                                names.push(name);
                            }
                            Event::End(e) => {
                                let end_name_res = qname_to_string(&e.name());
                                let Ok(end_name) = end_name_res else {
                                    return Self::non_decodable(end_name_res);
                                };
                                let Some(start_name) = names.pop() else {
                                    return Err(Error::IllFormed(IllFormedError::UnmatchedEndTag(
                                        end_name,
                                    )));
                                };
                                if start_name != end_name {
                                    return Err(Error::IllFormed(
                                        IllFormedError::MismatchedEndTag {
                                            expected: start_name,
                                            found: end_name,
                                        },
                                    ));
                                }
                            }
                            _ => (),
                        }

                        if names.is_empty() {
                            break;
                        }

                        nested_events.push(events[i]);
                    }
                    let el_children = Self::from_events(&nested_events[..])?;
                    children.push(Item::Element(Element {
                        name,
                        attributes,
                        children: el_children,
                    }));
                }
                Event::End(e) => {
                    // Should have been handled in Event::Start block. It's therefore an umatched end.
                    let name_res = qname_to_string(&e.name());
                    let Ok(name) = name_res else {
                        return Self::non_decodable(name_res);
                    };
                    return Err(Error::IllFormed(IllFormedError::UnmatchedEndTag(name)));
                }
                Event::Eof => {
                    // Should have been filtered by get_all_events
                    panic!("Internal Error: EoF found where EoF should never be.");
                }
            }
            i += 1;
        }

        Ok(children)
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = &match self {
            Self::Element(element) => element.to_string(),
            Self::EmptyElement(element) => element.to_string(),
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

fn get_name(start: &BytesStart) -> Result<String, FromUtf8Error> {
    qname_to_string(&start.name())
}

fn get_attributes(start: &BytesStart) -> Result<HashMap<String, String>, FromUtf8Error> {
    let attrs: Vec<Attribute> = start
        .attributes()
        .filter_map(|attr| {
            if attr.is_ok() {
                Some(attr.unwrap())
            } else {
                None
            }
        })
        .collect();

    let mut attributes = HashMap::with_capacity(attrs.len());

    for attr in attrs {
        let key = qname_to_string(&attr.key)?;
        let value = String::from_utf8((*attr.value).to_vec())?;
        attributes.insert(key, value);
    }

    Ok(attributes)
}

fn qname_to_string(qname: &QName) -> Result<String, FromUtf8Error> {
    u8_to_string(qname.as_ref())
}

fn u8_to_string(u8: &[u8]) -> Result<String, FromUtf8Error> {
    String::from_utf8(u8.to_vec())
}

fn get_all_events(xml: &str, trim: bool) -> Result<Vec<Event>, Error> {
    let mut events = Vec::new();

    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(trim);

    loop {
        match reader.read_event() {
            Err(err) => return Err(err),

            Ok(Event::Eof) => break,

            Ok(e) => events.push(e),
        };
    }

    Ok(events)
}
