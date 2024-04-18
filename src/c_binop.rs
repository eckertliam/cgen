use crate::{CExpr, Render};

#[derive(Debug, Clone)]
pub struct CBinOp {
    pub left: Box<CExpr>,
    pub op: &'static str,
    pub right: Box<CExpr>,
}

impl CBinOp {
    pub fn new(left: CExpr, op: &'static str, right: CExpr) -> Self {
        Self {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }
}

impl Render for CBinOp {
    fn render(&self) -> String {
        format!("{} {} {}", self.left.render(), self.op, self.right.render())
    }
}