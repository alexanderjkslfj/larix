pub use quick_xml::Error;

mod util;
pub use util::{parse, parse_trimmed, stringify};

mod item;
pub use item::*;

mod element;
pub use element::*;
