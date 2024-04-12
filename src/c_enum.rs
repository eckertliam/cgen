use crate::{CExpr, Render, CType};

pub struct CEnumDef {
    pub name: &'static str,
    pub variants: Vec<(&'static str, Option<CExpr>)>,
}

impl CEnumDef {
    pub fn new(name: &'static str) -> Self {
        CEnumDef {
            name,
            variants: Vec::new(),
        }
    }

    pub fn variant(&mut self, name: &'static str, value: Option<CExpr>) {
        self.variants.push((name, value));
    }
}

impl Render for CEnumDef {
    fn render(&self) -> String {
        let variants = self
            .variants
            .iter()
            .map(|(name, value)| {
                if let Some(value) = value {
                    format!("{} = {}", name, value.render())
                } else {
                    name.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join(", ");
        format!("typedef enum {{{}}} {};", variants, self.name)
    }
}

pub struct CEnumInit {
    pub ty: CType,
    pub name: &'static str,
    pub variant: &'static str,
}

impl CEnumInit {
    pub fn new(ty: CType, name: &'static str, variant: &'static str) -> Self {
        CEnumInit { ty, name, variant }
    }

    pub fn from_enum_def(def: &CEnumDef, name: &'static str, variant: &'static str) -> Self {
        CEnumInit {
            ty: CType::Enum(def.name.to_string()),
            name,
            variant,
        }
    }
}

impl Render for CEnumInit {
    fn render(&self) -> String {
        format!("{} {} = {};", self.ty.render(), self.name, self.variant)
    }
}