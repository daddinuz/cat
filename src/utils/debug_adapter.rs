use super::Debug;

pub struct DebugAdapter<T: Debug>(T);

impl<T: Debug> From<T> for DebugAdapter<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T: Debug> std::fmt::Debug for DebugAdapter<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(inner) = self;
        inner.fmt(f)
    }
}
