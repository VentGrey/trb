use std::default::Default;
use std::fmt;

//-- Estructura Stylesheet, su único elemento es un vector de reglas.
#[derive(PartialEq)]
pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

//-- Estructura Regla, toma dos parámetros los cuales son un vector selector
//-- y un vector declaración
#[derive(PartialEq)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

//-- Estructura Selector conformada por dos elementos, un selector simple y un
//-- "combinador".
#[derive(PartialEq, Eq)]
pub struct Selector {
    pub simple: Vec<SimpleSelector>,
    pub combinators: Vec<char>,
}

//-- Estructura Selector simple, toma 3 elementos, el nombre de la etiqueta
//-- id (para el identificador) y las clases (tomadas de cualquier HTML tag).
#[derive(PartialEq, Eq)]
pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub classes: Vec<String>,
}
