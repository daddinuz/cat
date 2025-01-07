pub trait Concat<S> {
    type Output;

    fn concat(self, other: S) -> Self::Output;
}

impl<T, I> Concat<I> for Vec<T>
where
    I: IntoIterator<Item = T>,
{
    type Output = Self;

    fn concat(mut self, other: I) -> Self::Output {
        self.extend(other);
        self
    }
}

impl Concat<String> for String {
    type Output = Self;

    fn concat(mut self, other: String) -> Self::Output {
        self += other.as_str();
        self
    }
}

impl Concat<&str> for String {
    type Output = Self;

    fn concat(mut self, other: &str) -> Self::Output {
        self += other;
        self
    }
}
