use super::Display;

pub struct DisplayAdapter<T: Display>(T);

impl<T: Display> From<T> for DisplayAdapter<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T: Display> std::fmt::Display for DisplayAdapter<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(inner) = self;
        inner.fmt(f)
    }
}
