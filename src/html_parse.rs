//-- Importamos nuestra biblioteca dom con todos sus elementos.
use dom::{AttrMap, ElementData, Node, NodeType};

use std::iter::Peekable;
use std::str::Chars;


//-- Estructura principal del parser HTML.
pub struct HtmlParser<'a> {
    chars: Peekable<Chars<'a>>,
    node_q: Vec<String>,
}

