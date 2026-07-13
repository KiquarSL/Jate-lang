/// Separated with other types for using primitive if not using method and wrapper if using methods
#[derive(Debug, Clone, Copy)]
pub enum PrimitiveType {
    Short,
    Int,
    Long,
    Float,
    Double,
}

/// All variants of types
#[derive(Debug, Clone)]
pub enum Type {
    Unknown,
    /// Using for functions/methods with no return value
    /// Not using as type in syntax
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
