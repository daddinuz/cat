use crate::stack::Stack;

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

impl<S> Concat<()> for S
where
    S: Stack,
{
    type Output = S;

    fn concat(self, _: ()) -> Self::Output {
        self
    }
}

impl<T, H, S> Concat<(T, H)> for S
where
    T: Stack,
    S: Stack + Concat<T>,
{
    type Output = (S::Output, H);

    fn concat(self, (tail, head): (T, H)) -> Self::Output {
        (self.concat(tail), head)
    }
}

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

impl<T> Contains<T> for ()
where
    Self: Stack,
{
    fn contains(&self, _: T) -> bool {
        false
    }
}

impl<T, H> Contains<H> for (T, H)
where
    Self: Stack,
    H: PartialEq,
    T: Contains<H>,
{
    fn contains(&self, value: H) -> bool {
        let (tail, head) = self;
        head == &value || tail.contains(value)
    }
}

pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

impl<T> IsEmpty for Vec<T> {
    fn is_empty(&self) -> bool {
        Vec::is_empty(self)
    }
}

impl<T, const N: usize> IsEmpty for [T; N] {
    fn is_empty(&self) -> bool {
        N == 0
    }
}

impl<T> IsEmpty for &'_ [T] {
    fn is_empty(&self) -> bool {
        <[T]>::is_empty(self)
    }
}

impl IsEmpty for String {
    fn is_empty(&self) -> bool {
        String::is_empty(self)
    }
}

impl IsEmpty for &str {
    fn is_empty(&self) -> bool {
        str::is_empty(self)
    }
}

impl IsEmpty for () {
    fn is_empty(&self) -> bool {
        true
    }
}

impl<T, H> IsEmpty for (T, H)
where
    Self: Stack,
{
    fn is_empty(&self) -> bool {
        false
    }
}

impl<T: IsEmpty> IsEmpty for &T {
    fn is_empty(&self) -> bool {
        (*self).is_empty()
    }
}

pub trait Len {
    fn len(&self) -> usize;
}

impl<T> Len for Vec<T> {
    fn len(&self) -> usize {
        Vec::len(self)
    }
}

impl<T, const N: usize> Len for [T; N] {
    fn len(&self) -> usize {
        N
    }
}

impl<T> Len for &'_ [T] {
    fn len(&self) -> usize {
        <[T]>::len(self)
    }
}

impl Len for String {
    fn len(&self) -> usize {
        String::len(self)
    }
}

impl Len for &str {
    fn len(&self) -> usize {
        str::len(self)
    }
}

impl Len for () {
    fn len(&self) -> usize {
        0
    }
}

impl<T: Len, H> Len for (T, H)
where
    Self: Stack,
{
    fn len(&self) -> usize {
        let (l, _) = self;
        l.len() + 1
    }
}

impl<T: Len> Len for &T {
    fn len(&self) -> usize {
        (*self).len()
    }
}
