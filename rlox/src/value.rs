#[derive(Debug, Clone, Copy)]
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
}
