use super::DisplayAdapter;

pub trait Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;

    fn display<'a>(&'a self) -> DisplayAdapter<&'a Self>
    where
        &'a Self: Display,
    {
        DisplayAdapter::from(self)
    }
}

impl<T: Display> Display for &T {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (*self).fmt(f)
    }
}
