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
#[derive(PartialEq)]
pub struct Declaration {
    pub property: String,
    pub value: Value,
}
//-- Enum de tipo valor que puede tomar solo uno de tres tipos, Color de tipo
//-- color, Length que toma un flotante y una unidad y un último valor opcional
#[derive(PartialEq)]
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
//-- PROGRAMACIÓN ORIENTADA A OBJETOS INCOMING! (Está bien fea XD)
impl Stylesheet {
    pub fn new(rules: Vec<Rule>) -> Stylesheet {
        Stylesheet { rules }
    }
}
//-- Valor por defecto de Stylesheet
impl Default for Stylesheet {
    fn default() -> Self {
        Stylesheet { rules: Vec::new() }
    }
}
//-- Símbolos de depuración para las Stylesheet
// FIXME: Más de tres niveles de sangría.
impl fmt::Debug for Stylesheet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rule_result = String::new();
        for rule in &self.rules {
            if rule_result.len() > 0 {
                rule_result.push_str("\n\n");
            }
            rule_result.push_str(&format!("{:?}", rule));
        }
        write!(f, "{}", rule_result)
    }
}
//-- Objeto Rule
// FIXME: Más de 80 columnas de código
impl Rule {
    pub fn new(selectors: Vec<Selector>, declarations: Vec<Declaration>) -> Rule {
        Rule {
            selectors,
            declarations,
        }
    }
}
//-- Valor defecto para Rule
impl Default for Rule {
    fn default() -> Self {
        Rule {
            selectors: Vec::new(),
            declarations: Vec::new(),
        }
    }
}
//-- Símbolos de depuración para Rule
impl fmt::Debug for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sel_result = String::new();
        let mut decl_result = String::new();
        let tab = "    ";

        for selector in &self.selectors {
            if sel_result.len() > 0 {
                sel_result.push_str(", ");
            }
            sel_result.push_str(&format!("{:?}", selector));
        }

        for declaration in &self.declarations {
            decl_result.push_str(tab);
            decl_result.push_str(&format!("{:?}", declaration));
            decl_result.push('\n');
        }

        write!(f, "{} {{\n{}}}", sel_result, decl_result)
    }
}
//-- Objeto Selector
// FIXME: Me pasé de la columna 80 otra vez.
impl Selector {
    pub fn new(simple: Vec<SimpleSelector>, combinators: Vec<char>) -> Selector {
        Selector {
            simple,
            combinators,
        }
    }
}
//-- Valor por defecto para Selector
impl Default for Selector {
    fn default() -> Self {
        Selector {
            simple: Vec::new(),
            combinators: Vec::new(),
        }
    }
}
//-- Símbolos de depuración para el selector
impl fmt::Debug for Selector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        for sel in &self.simple {
            if result.len() > 0 {
                result.push_str(", ");
            }
            result.push_str(&format!("{:?}", sel));
        }

        write!(f, "{}", result)
    }
}
//-- Objeto Selector Simple
// TODO: Arreglar el cagadero de nombres que hice, me pasé de turbo verga
impl SimpleSelector {
    pub fn new(
        tag_name: Option<String>,
        id: Option<String>,
        classes: Vec<String>,
    ) -> SimpleSelector {
        SimpleSelector {
            tag_name,
            id,
            classes,
        }
    }
}
//-- Valores defecto del selector simple
impl Default for SimpleSelector {
    fn default() -> Self {
        SimpleSelector {
            tag_name: None,
            id: None,
            classes: Vec::new(),
        }
    }
}
//-- Símbolos de depuración para el Selector simple
impl fmt::Debug for SimpleSelector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        match self.tag_name {
            Some(ref t) => result.push_str(t),
            None => {}
        }

        match self.id {
            Some(ref s) => {
                result.push('#');
                result.push_str(s);
            }
            None => {}
        }

        for class in &self.classes {
            result.push('.');
            result.push_str(class);
        }

        write!(f, "{}", result)
    }
}
//-- Objeto declaración
impl Declaration {
    pub fn new(property: String, value: Value) -> Declaration {
        Declaration { property, value }
    }
}
//-- Valores por defecto de declaración
impl Default for Declaration {
    fn default() -> Self {
        Declaration {
            property: String::from(""),
            value: Value::Other(String::from("")),
        }
    }
}
//-- Par de objetos para símbolos de depuración de Declaración y Valor
impl fmt::Debug for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {:?}", self.property, self.value)
    }
}
impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Color(ref c) => write!(f, "{:?}", c),
            Value::Length(l, _) => write!(f, "{:?}", l),
            Value::Other(ref s) => write!(f, "{:?}", s),
        }
    }
}
//-- Objeto Color
impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color { r, g, b, a }
    }
}
//-- Valores por defecto para Color
impl Default for Color {
    fn default() -> Self {
        Color::new(1.0, 1.0, 1.0, 1.0)
    }
}
//-- Símbolos de depuración para Color
impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "r: {} g: {} b: {} a: {}", self.r, self.g, self.b, self.a)
    }
}
