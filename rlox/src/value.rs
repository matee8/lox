use core::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Bool(bool),
    Number(f64),
    Nil,
}

impl Value {
    pub const fn as_bool(&self) -> Option<bool> {
        if let Self::Bool(value) = *self {
            Some(value)
        } else {
            None
        }
    }

    pub const fn as_number(&self) -> Option<f64> {
        if let Self::Number(value) = *self {
            Some(value)
        } else {
            None
        }
    }

    pub const fn is_falsey(&self) -> bool {
        match *self {
            Self::Bool(value) => !value,
            Self::Number(_) => false,
            Self::Nil => true,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Bool(value) => {
                write!(f, "{}", if value { "true" } else { "false" })
            }
            Self::Number(value) => write!(f, "{value}"),
            Self::Nil => write!(f, "nil"),
        }
    }
}
