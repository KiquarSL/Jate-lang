#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Keyword {
    Fn,
    While,
    For,
    Class,
    Trait,
    Pub,
    Pkg,
    Use,
    Return,
    In,
}

impl Keyword {
    pub fn to_keyword(input: &str) -> Option<Keyword> {
        match input {
            "fn" => Some(Self::Fn),
            "use" => Some(Self::Use),
            "pub" => Some(Self::Pub),
            "return" => Some(Self::Return),
            "pkg" => Some(Self::Pkg),
            "class" => Some(Self::Class),
            "trait" => Some(Self::Trait),
            "for" => Some(Self::For),
            "while" => Some(Self::While),
            "in" => Some(Self::In),
            _ => None,
        }
    }
}
