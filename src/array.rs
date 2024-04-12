use crate::{CExpr, CType, Render};

#[derive(Debug, Clone)]
pub struct ArrayLiteral {
    pub data: Vec<CExpr>,
}

impl ArrayLiteral {
    pub fn new() -> Self {
        ArrayLiteral { data: Vec::new() }
    }

    pub fn push(&mut self, expr: CExpr) {
        self.data.push(expr);
    }
}

impl Render for ArrayLiteral {
    fn render(&self) -> String {
        let inner = self
            .data
            .iter()
            .map(|expr| expr.render())
            .collect::<Vec<String>>()
            .join(", ");
        format!("{{{}}}", inner)
    }
}

pub struct ArrayInit {
    pub ty: CType,
    pub name: String,
    pub expr: ArrayLiteral,
}

impl ArrayInit {
    pub fn new(ty: CType, name: &str) -> Self {
        ArrayInit {
            ty,
            name: name.to_string(),
            expr: ArrayLiteral::new(),
        }
    }

    pub fn push(&mut self, expr: CExpr) {
        self.expr.push(expr);
    }
}

impl Render for ArrayInit {
    fn render(&self) -> String {
        format!(
            "{} {}[] = {};",
            self.ty.render(),
            self.name,
            self.expr.render()
        )
    }
}
