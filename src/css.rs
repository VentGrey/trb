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

//-- Estructura de declaración que toma 2 elementos, propiedad que es un String
//-- y un tipo de dato llamado Value
pub struct Declaration {
    pub property: String,
    pub value: Value,
}

//-- Enum de tipo valor que puede tomar solo uno de tres tipos, Color de tipo
//-- color, Length que toma un flotante y una unidad y un último valor opcional
pub enum Value {
    Color(Color),
    Length(f32, Unit),
    Other(String),
}

//-- Enum de tipo unidad para manejar todas las unidades aceptables en el CSS
#[derive(PartialEq)]
pub enum Unit {
    Em,
    Ex,
    Ch,
    Rem,
    Vh,
    Vw,
    Vmin,
    Vmax,
    Px,
    Mm,
    Q,
    Cm,
    In,
    Pt,
    Pc,
    Pct,
}

//-- Estructura de tipo Color que toma un valor flotante del espectro RGBA
//-- utilizado en CSS
#[derive(PartialEq, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
