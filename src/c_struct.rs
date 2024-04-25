use crate::CExpr;

use super::{CType, Render};

// a template to generate a C struct
#[derive(Debug, Clone)]
pub struct StructDef {
    pub name: &'static str,
    pub fields: Vec<(&'static str, CType)>,
}

impl StructDef {
    pub fn new(name: &'static str) -> Self {
        StructDef {
            name,
            fields: Vec::new(),
        }
    }
}

#[macro_export]
macro_rules! struct_def {
    ($name:ident { $($field_name:ident: $field_type:expr),* }) => {
        StructDef {
            name: stringify!($name),
            fields: vec![
                $((stringify!($field_name), $field_type)),*
            ],
        }
    };
}

impl Render for StructDef {
    fn render(&self) -> String {
        let fields = self
            .fields
            .iter()
            .map(|(name, ty)| format!("{} {}", ty.render(), name))
            .collect::<Vec<String>>()
            .join("; ");
        format!(
            "typedef struct {} {{{};}} {};",
            self.name, fields, self.name
        )
    }
}

// a template for initializing a struct
#[derive(Debug, Clone)]
pub struct StructInit {
    // the type of the struct
    pub ty: CType,
    // the identifier of the struct
    pub name: &'static str,
    pub fields: Vec<CExpr>,
}

impl StructInit {
    pub fn new(ty: CType, name: &'static str) -> Self {
        StructInit {
            ty,
            name,
            fields: Vec::new(),
        }
    }

    pub fn add_field(&mut self, field: CExpr) {
        self.fields.push(field);
    }

    pub fn from_struct_def(def: &StructDef, name: &'static str, fields: &[CExpr]) -> Self {
        let mut init = StructInit::new(CType::Struct(def.name.to_string()), name);
        for i in 0..def.fields.len() {
            if let Some(field) = fields.get(i) {
                init.add_field(field.clone());
            } else {
                panic!("missing field value for field {}", def.fields[i].0)
            }
        }
        init
    }
}

impl Render for StructInit {
    fn render(&self) -> String {
        let fields = self
            .fields
            .iter()
            .map(|f| f.render())
            .collect::<Vec<String>>()
            .join(", ");
        format!("{} {} = {{{}}};", self.ty.render(), self.name, fields)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_struct_def() {
        let s = struct_def!(Person {
            name: CType::String,
            age: CType::Int
        });
        assert_eq!(
            s.render(),
            "typedef struct Person {char* name; int age;} Person;"
        );
    }
}
