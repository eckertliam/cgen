mod c_type;
pub use c_type::CType;
mod c_expr;
pub use c_expr::CExpr;
mod c_struct;
pub use c_struct::{StructDef, StructInit};
mod c_enum;
pub use c_enum::{CEnumDef, CEnumInit};
mod c_func;
pub use c_func::CFunc;
mod array;
pub use array::{ArrayInit, ArrayLiteral};
mod c_if;
pub use c_if::CIf;
mod c_binop;
pub use c_binop::CBinOp;
mod c_unop;
pub use c_unop::CUnOp;
mod c_stmt;
pub use c_stmt::CStmt;
mod compiler;

pub trait Render {
    fn render(&self) -> String;
}