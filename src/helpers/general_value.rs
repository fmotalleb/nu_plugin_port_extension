use nu_protocol::{Span, Value};

pub trait AsValue {
    fn as_value(self, span: Span) -> Value;
}

impl AsValue for Option<Value> {
    fn as_value(self, span: Span) -> Value {
        self.unwrap_or(Value::nothing(span))
    }
}
impl<T: AsValue> AsValue for Option<T> {
    fn as_value(self, span: Span) -> Value {
        self.map(|f| f.as_value(span))
            .unwrap_or(Value::nothing(span))
    }
}
impl<T: AsValue + Clone> AsValue for &T {
    fn as_value(self, span: Span) -> Value {
        self.to_owned().as_value(span)
    }
}

impl AsValue for u16 {
    fn as_value(self, span: Span) -> Value {
        Value::int(self.into(), span)
    }
}

impl AsValue for u32 {
    fn as_value(self, span: Span) -> Value {
        Value::int(self.into(), span)
    }
}

impl AsValue for String {
    fn as_value(self, span: Span) -> Value {
        Value::string(self, span)
    }
}
impl AsValue for &str {
    fn as_value(self, span: Span) -> Value {
        Value::string(self, span)
    }
}

impl<T: AsValue> AsValue for Vec<T> {
    fn as_value(self, span: Span) -> Value {
        Value::list(
            self.into_iter().map(|item| item.as_value(span)).collect(),
            span,
        )
    }
}
