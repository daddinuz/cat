pub trait Contains<T> {
    fn contains(&self, value: T) -> bool;
}

impl<T> Contains<T> for Vec<T>
where
    T: PartialEq,
{
    fn contains(&self, value: T) -> bool {
        <[T]>::contains(self.as_slice(), &value)
    }
}

impl<T, const N: usize> Contains<T> for [T; N]
where
    T: PartialEq,
{
    fn contains(&self, value: T) -> bool {
        <[T]>::contains(self.as_slice(), &value)
    }
}

impl<T> Contains<T> for &'_ [T]
where
    T: PartialEq,
{
    fn contains(&self, value: T) -> bool {
        <[T]>::contains(self, &value)
    }
}

impl Contains<char> for String {
    fn contains(&self, value: char) -> bool {
        str::contains(self, value)
    }
}

impl Contains<&str> for String {
    fn contains(&self, value: &str) -> bool {
        str::contains(self, value)
    }
}

impl Contains<char> for &str {
    fn contains(&self, value: char) -> bool {
        str::contains(self, value)
    }
}

impl Contains<&str> for &str {
    fn contains(&self, value: &str) -> bool {
        str::contains(self, value)
    }
}
