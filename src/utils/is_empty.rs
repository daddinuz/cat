pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

impl<T: IsEmpty> IsEmpty for &T {
    fn is_empty(&self) -> bool {
        (*self).is_empty()
    }
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

impl<T> IsEmpty for &[T] {
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
