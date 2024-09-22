#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Number,
    Boolean,
    String,
    Function(Vec<Type>, Box<Type>),
    Struct(Vec<(String, Type)>),
    Generic(String),
    HKT(String, Vec<Type>),
}

pub struct TypeEnv {
    types: std::collections::HashMap<String, Type>,
}

impl TypeEnv {
    pub fn new() -> Self {
        TypeEnv {
            types: std::collections::HashMap::new(),
        }
    }

    pub fn add_type(&mut self, name: String, ty: Type) {
        self.types.insert(name, ty);
    }

    pub fn get_type(&self, name: &str) -> Option<&Type> {
        self.types.get(name)
    }
}