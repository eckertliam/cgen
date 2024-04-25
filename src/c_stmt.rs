use crate::{
    ArrayInit, CEnumDef, CEnumInit, CExpr, CFunc, CIf, CType, Render, StructDef, StructInit,
};

#[derive(Debug, Clone)]
pub enum CStmt {
    Expr(CExpr),
    StructInit(StructInit),
    StructDef(StructDef),
    Func(CFunc),
    Assign(CExpr, CExpr),
    Return(CExpr),
    If(CIf),
    VarDecl(CType, String, Option<CExpr>),
    ArrayInit(ArrayInit),
    EnumDef(CEnumDef),
    EnumInit(CEnumInit),
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
            CStmt::VarDecl(ty, name, expr) => {
                if let Some(expr) = expr {
                    format!("{} {} = {};", ty.render(), name, expr.render())
                } else {
                    format!("{} {};", ty.render(), name)
                }
            }
            CStmt::ArrayInit(array) => array.render(),
            CStmt::EnumDef(def) => def.render(),
            CStmt::EnumInit(init) => init.render(),
        }
    }
}
