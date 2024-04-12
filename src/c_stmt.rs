use crate::{
    CExpr, CFunc, CIf, Render, StructDef, StructInit
};

pub enum CStmt {
    Expr(CExpr),
    StructInit(StructInit),
    StructDef(StructDef),
    Func(CFunc),
    Assign(CExpr, CExpr),
    Return(CExpr),
    If(CIf),
}

impl Render for CStmt {
    fn render(&self) -> String {
        match self {
            CStmt::Expr(expr) => format!("{};", expr.render()),
            CStmt::StructDef(def) => def.render(),
            CStmt::StructInit(init) => init.render(),
            CStmt::Func(func) => func.render(),
            CStmt::Assign(lhs, rhs) => format!("{} = {};", lhs.render(), rhs.render()),
            CStmt::Return(expr) => format!("return {};", expr.render()),
            CStmt::If(if_stmt) => if_stmt.render(),
        }
    }
}