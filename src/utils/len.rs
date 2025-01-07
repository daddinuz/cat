use super::IsEmpty;

pub trait Len: IsEmpty {
    fn len(&self) -> usize;
}

impl<T: Len> Len for &T {
    fn len(&self) -> usize {
        (*self).len()
    }
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

impl<T> Len for &[T] {
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
