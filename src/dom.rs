//--- Imports de la biblioteca estándar
use std::collections::{HashMap, HashSet};
use std::fmt;

//-- Estructura Nodo, posee dos elementos
//-- 1- children (Vector de Nodos)
//-- 2- node_type (Dato de tipo NodeType)
#[derive(PartialEq, Eq)]
pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

//-- Enum para definir el tipo de nodo, el nodo puede tener solo UNO de tres
//-- posibles tipos:

//-- 1- Nodo de Texto (String)
//-- 2- Nodo Elemento (Tipo de dato ElementData)
//-- 3- Nodo Comentario (String)
#[derive(PartialEq, Eq, Clone)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String),
}

//-- Estructura ElementData (Mencionada arriba) conformada por dos elementos:
//-- 1- tag_name (String)
//-- 2- attributes: Tipo de dato AttrMap
#[derive(PartialEq, Eq, Clone)]
pub struct ElementData {
    pub tag_name: String,
    attributes: AttrMap,
}

//-- POO extraña usando IMPLS :D
//-- Esto básicamente es un objeto, nada mas que explicar.
impl ElementData {
    pub fn new(tag_name: String, attributes: AttrMap) -> ElementData {
        ElementData {
            tag_name,
            attributes,
        }
    }

    pub fn get_id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    pub fn get_classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(s) => s.split(' ').collect(),
            None => HashSet::new(),
        }
    }
}

//-- Declaración de un tipo de dato nuevo llamado AttrMap, el cual está
//-- Conformado por un HashMap de dos Strings
pub type AttrMap = HashMap<String, String>;

//-- Otra implementación de la estructura Nodo como Objeto
impl Node {
    pub fn new(node_type: NodeType, children: Vec<Node>) -> Node {
        Node {
            node_type,
            children,
        }
    }
}

//-- Información de Depuración
impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.node_type)
    }
}

//-- Información de Depuración
impl fmt::Debug for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NodeType::Text(ref t) | NodeType::Comment(ref t) => write!(f, "{}", t),
            NodeType::Element(ref e) => write!(f, "{:?}", e),
        }
    }
}

//-- Información de Depuración
impl fmt::Debug for ElementData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut attributes_string = String::new();

        for (attr, value) in self.attributes.iter() {
            attributes_string.push_str(&format!(" {}=\"{}\"", attr, value));
        }
        write!(f, "<{},{}>", self.tag_name, attributes_string)
    }
}

/* Función para imprimir de forma estética
La función toma 2 argumentos, el primero es un argumento llamado n que es
la referencia a un nodo y el segundo es un argumento llamado indent_size el
cual coma un tipo de dato usize (tamaño) */

pub fn pretty_print(n: &Node, indent_size: usize) {
    let indent = (0..indent_size).map(|_| " ").collect::<String>();

    match n.node_type {
        NodeType::Element(ref e) => println!("{}{:?}", indent, e),
        NodeType::Text(ref t) => println!("{}{}", indent, t),
        NodeType::Comment(ref c) => println!("{}<!--{}-->", indent, c),
    }

    for child in n.children.iter() {
        pretty_print(&child, indent_size + 2);
    }

    match n.node_type {
        NodeType::Element(ref e) => println!("{}<{}/>", indent, e.tag_name),
        _ => {}
    }
}
