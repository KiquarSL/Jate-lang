/// Separated with other types for using primitive if not using method and wrapper if using methods
#[derive(Debug, Clone, Copy)]
pub enum PrimitiveType {
    Short,
    Int,
    Long,
    Float,
    Double,
}

#[derive(Debug, Clone)]
pub enum Type {
    Unknown,
    Void,
    Class(String),
    Trait(String),
    /// `[type]`
    Array(Box<Type>),
    /// `type?`
    Nullable(Box<Type>),
    Primitive {
        ty: PrimitiveType,
        boxed: bool,
    },
}
