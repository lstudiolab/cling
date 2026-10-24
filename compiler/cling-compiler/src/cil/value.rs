#[derive(Clone, Debug, PartialEq)]
pub enum Value { Int(i64), Bool(bool), String(String), Void }

impl std::fmt::Display for Value {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(value) => write!(formatter, "{value}"),
            Value::Bool(value) => write!(formatter, "{value}"),
            Value::String(value) => write!(formatter, "{value}"),
            Value::Void => Ok(()),
        }
    }
}
