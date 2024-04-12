use crate::{CStmt, CType, Render};

pub struct CFunc {
    pub name: &'static str,
    pub ret_ty: CType,
    pub params: Vec<(&'static str, CType)>,
    pub body: Vec<CStmt>,
}

impl CFunc {
    pub fn new(name: &'static str, ret_ty: CType) -> Self {
        CFunc {
            name,
            ret_ty,
            params: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn param(&mut self, name: &'static str, ty: CType) {
        self.params.push((name, ty));
    }

    pub fn stmt(&mut self, stmt: CStmt) {
        self.body.push(stmt);
    }
}

impl Render for CFunc {
    fn render(&self) -> String {
        let params = self
            .params
            .iter()
            .map(|(name, ty)| format!("{} {}", ty.render(), name))
            .collect::<Vec<String>>()
            .join(", ");
        let body = self
            .body
            .iter()
            .map(|stmt| stmt.render())
            .collect::<Vec<String>>()
            .join("\n");
        format!(
            "{} {}({}) {{\n{}}}",
            self.ret_ty.render(),
            self.name,
            params,
            body
        )
    }
}
