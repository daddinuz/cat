use super::DebugAdapter;

pub trait Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;

    fn debug<'a>(&'a self) -> DebugAdapter<&'a Self>
    where
        &'a Self: Debug,
    {
        DebugAdapter::from(self)
    }
}

impl<T: Debug> Debug for &T {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (*self).fmt(f)
    }
}
