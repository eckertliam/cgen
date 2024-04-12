use crate::{CExpr, CStmt, Render};

pub struct CIf {
    pub cond: CExpr,
    pub body: Vec<CStmt>,
    pub else_body: Option<Vec<CStmt>>,
}

impl CIf {
    pub fn new(cond: CExpr) -> Self {
        CIf {
            cond,
            body: Vec::new(),
            else_body: None,
        }
    }

    pub fn stmt(&mut self, stmt: CStmt) {
        self.body.push(stmt);
    }

    pub fn else_stmt(&mut self, stmt: CStmt) {
        if self.else_body.is_none() {
            self.else_body = Some(Vec::new());
        } else {
            self.else_body.as_mut().unwrap().push(stmt);
        }
    }
}

impl Render for CIf {
    fn render(&self) -> String {
        let body = self
            .body
            .iter()
            .map(|stmt| stmt.render())
            .collect::<Vec<String>>()
            .join("\n");
        let else_body = if let Some(else_body) = &self.else_body {
            let else_body = else_body
                .iter()
                .map(|stmt| stmt.render())
                .collect::<Vec<String>>()
                .join("\n");
            format!("else {{\n{}}}", else_body)
        } else {
            String::new()
        };
        format!("if ({}) {{\n{}}}\n{}", self.cond.render(), body, else_body)
    }
}
