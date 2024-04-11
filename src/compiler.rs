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

    pub fn compile(&self, file_name: &str) {
        use std::process::Command;

        self.render_to_file(file_name);
        let output = Command::new("gcc")
            .arg(file_name)
            .arg("-o")
            .arg("output")
            .output()
            .expect("failed to compile");

        if !output.status.success() {
            panic!("failed to compile");
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
    use crate::{CExpr, CType, CStmt, CFunc};

    #[test]
    fn test_compiler() {
        let mut compiler = Compiler::new();
        let mut main = CFunc::new("main", CType::Int);
        main.stmt(CStmt::Return(CExpr::Int(0)));
        compiler.stmt(CStmt::Func(main));
        compiler.compile("hello.c");
        assert_eq!(std::fs::read_to_string("hello.c").unwrap(), "int main() {\nreturn 0;}");
    }
}