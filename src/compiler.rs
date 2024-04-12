use crate::{CStmt, Render};

pub struct Compiler {
    body: Vec<CStmt>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            body: Vec::new(),
        }
    }

    pub fn stmt(&mut self, stmt: CStmt) {
        self.body.push(stmt);
    }

    pub fn render_to_file(&self, file_name: &str) {
        use std::fs::File;
        use std::io::Write;

        let mut file = File::create(file_name).unwrap();
        file.write_all(self.render().as_bytes()).unwrap();
    }

    pub fn compile(&self, file_name: &str) -> Result<(), String> {
        use std::process::Command;

        self.render_to_file(file_name);
        let output = Command::new("gcc")
            .arg(file_name)
            .arg("-o")
            .arg("output")
            .output()
            .expect("failed to compile");

        if !output.status.success() {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        } else {
            Ok(())
        }
    }
}

// we need to implement the Render trait for Compiler so that we can use the render method on it to generate the C code
// we then write the C code to a file and compile it using the C compiler
impl Render for Compiler {
    fn render(&self) -> String {
        self.body.iter().map(|stmt| stmt.render()).collect::<Vec<String>>().join("\n")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{CExpr, CType, CStmt, CFunc, StructDef, StructInit, struct_def};

    #[test]
    fn test_compiler() {
        let mut compiler = Compiler::new();
        let mut main = CFunc::new("main", CType::Int);
        let person = struct_def!(Person {
            name: CType::String,
            age: CType::Int
        });
        compiler.stmt(CStmt::StructDef(person.clone()));
        main.stmt(CStmt::StructInit(StructInit::from_struct_def(&person, "p", &[
            CExpr::Str("John"),
            CExpr::Int(20),
        ])));
        main.stmt(CStmt::Return(CExpr::Int(0)));
        compiler.stmt(CStmt::Func(main));
        let res = compiler.compile("hello.c");
        assert!(res.is_ok());
    }
}