use crate::Render;

pub enum CType {
    Int,
    Float,
    Double,
    Char,
    String,
    Void,
    Struct(String),
    Union(String),
    Enum(String),
    Pointer(Box<CType>),
    Array(Box<CType>, usize),
    Function {
        return_type: Box<CType>,
        arguments: Vec<CType>,
    },
}

impl Render for CType {
    fn render(&self) -> String {
        match self {
            CType::Int => "int".to_string(),
            CType::Float => "float".to_string(),
            CType::Double => "double".to_string(),
            CType::Char => "char".to_string(),
            CType::String => "char*".to_string(),
            CType::Void => "void".to_string(),
            CType::Struct(name) => format!("{}", name),
            CType::Union(name) => format!("{}", name),
            CType::Enum(name) => format!("{}", name),
            CType::Pointer(t) => format!("{}*", t.render()),
            CType::Array(t, size) => format!("{}[{}]", t.render(), size),
            CType::Function { return_type, arguments } => {
                let args = arguments.iter().map(|t| t.render()).collect::<Vec<String>>().join(", ");
                format!("{}(*)({})", return_type.render(), args)
            }
        }
    }
}