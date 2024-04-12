use crate::{ArrayLiteral, Render};

#[derive(Debug, Clone)]
pub enum CExpr {
    Int(i32),
    Float(f32),
    Double(f64),
    Long(i64),
    Char(char),
    Str(&'static str),
    Ident(&'static str),
    FieldAccess(Box<CExpr>, &'static str),
    Array(ArrayLiteral),
    ArrayAccess(Box<CExpr>, Box<CExpr>),
    FuncCall { name: Box<CExpr>, args: Vec<CExpr> },
}

impl Render for CExpr {
    fn render(&self) -> String {
        match self {
            CExpr::Int(i) => i.to_string(),
            CExpr::Float(f) => f.to_string(),
            CExpr::Double(d) => d.to_string(),
            CExpr::Long(l) => l.to_string(),
            CExpr::Char(c) => format!("'{}'", c),
            CExpr::Str(s) => format!("\"{}\"", s),
            CExpr::Ident(i) => i.to_string(),
            CExpr::FieldAccess(expr, field) => format!("{}.{}", expr.render(), field),
            CExpr::Array(array) => array.render(),
            CExpr::ArrayAccess(array, index) => format!("{}[{}]", array.render(), index.render()),
            CExpr::FuncCall { name, args } => {
                let args = args
                    .iter()
                    .map(|arg| arg.render())
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("{}({})", name.render(), args)
            }
        }
    }
}
