use crate::{CExpr, Render};

#[derive(Debug, Clone)]
pub struct CUnOp {
    pub op: &'static str,
    pub expr: Box<CExpr>,
}

impl CUnOp {
    pub fn new(op: &'static str, expr: CExpr) -> Self {
        Self {
            op,
            expr: Box::new(expr),
        }
    }
}

impl Render for CUnOp {
    fn render(&self) -> String {
        format!("{}{}", self.op, self.expr.render())
    }
}